use super::Error;

use super::pub_structs::{
    ShorthandTransaction,
    DartProfile,
    Platform,
    PageName,
    SATS,
    Sats,
    Btc,
    Usd,
};
use super::threads::{call_thread, Threads, WalletMethod};
use super::wallet::{Transactions, Transaction};
use super::structs::DateTime;
use super::web5::{Profile, ShorthandConversation, Conversation, Message};

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

use simple_database::KeyValueStore;

use serde::{Serialize, Deserialize};
use serde_json::json;

use bdk::bitcoin::{Network, Address};
use bdk::bitcoin::hash_types::Txid;

use num_format::{Locale, ToFormattedString}; 

// USED FOR TESTING PURPOSES ONLY //
use rand::{distributions::Alphanumeric, Rng}; 
use secp256k1::rand;
use web5_rust::dids::Did;
use crate::api::state::rand::thread_rng;
use crate::api::web5;
// USED FOR TESTING PURPOSES ONLY //

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Field {
    Path(Option<PathBuf>),
    Price(Option<f64>),
    Internet(Option<bool>),
    Platform(Option<Platform>),

    Transactions(Option<Transactions>),
    Balance(Option<Sats>),

    Profile(Option<Profile>),
    ProfileAddress(Option<String>),

    Conversations(Option<Vec<Conversation>>),
    Users(Option<Vec<Profile>>),
    NewMessage(Option<Message>),

    Test(Option<u64>)
}

impl Field {
    pub fn into_bytes(&self) -> Vec<u8> {
        format!("{:?}", self).split("(").collect::<Vec<&str>>()[0].as_bytes().to_vec()
    }
}

#[derive(Clone)]
pub struct State {
    store: Box<dyn KeyValueStore>,
}

impl State {
    pub async fn new<KVS: KeyValueStore + 'static>(
        path: PathBuf,
    ) -> Result<Self, Error> {
        Ok(State{
            store: Box::new(KVS::new(path).await?)
        })
    }

    pub async fn set(&self, field: Field) -> Result<(), Error> {
        self.store.set(&field.into_bytes(), &serde_json::to_vec(&field)?).await?;
        Ok(())
    }

    pub async fn get_raw(&self, field: &Field) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.store.get(&field.into_bytes()).await?)
    }

    pub async fn get_o<T: for <'a> Deserialize<'a>>(&self, field: &Field) -> Result<Option<T>, Error> {
        Ok(self.get_raw(field).await?.map(|b|
            serde_json::from_slice::<Option<T>>(&b)
        ).transpose()?.flatten())
    }

    pub async fn get_or_default<T: for <'a> Deserialize<'a> + Default>(&self, field: &Field) -> Result<T, Error> {
        Ok(self.get_o(field).await?.unwrap_or_default())
    }

    pub async fn get<T: for <'a> Deserialize<'a>>(&self, field: &Field) -> Result<T, Error> {
        self.get_o(field).await?.ok_or(Error::not_found("State.get_f", &format!("Value not found for field: {:?}", field)))
    }
}

#[derive(Clone)]
pub struct StateManager {
    state: State,
}

impl StateManager {
    pub fn new(state: State) -> Self {
        StateManager{state}
    }

    #[async_backtrace::framed]
    pub async fn get(&mut self, page: PageName) -> Result<String, Error> {
        let page_state = match page {
            PageName::BitcoinHome => self.bitcoin_home().await,
            PageName::ViewTransaction(txid) => self.view_transaction(txid).await,
            PageName::Receive => self.receive().await,
            PageName::Send(address) => self.send(&address).await,
            PageName::Amount(amount) => self.amount(amount).await,
            PageName::Speed(amount) => self.speed(amount).await,
            PageName::Confirm(address, amount, fee) => self.confirm(address, amount, fee).await,
            PageName::Success(tx) => self.success(tx).await,
            PageName::MyProfile(init) => self.my_profile(init).await,
            PageName::UserProfile(init, profile) => self.user_profile(init, profile).await,
            PageName::MessagesHome => self.messages_home().await,
            PageName::ChooseRecipient => self.choose_recipient().await,
            PageName::ConversationInfo(record) => self.conversation_info(record).await,
            PageName::CurrentConversation(record, members) => self.current_conversation(record, members).await,
            PageName::Scan => self.scan_qr().await,
            PageName::Test(_) => self.test().await,
        };
        Ok(serde_json::to_string(&json!({
            "connected": self.state.get_or_default::<bool>(&Field::Internet(None)).await?,
            "state": match page_state {
                Err(Error::NoInternet{backtrace: _}) => String::new(),
                Err(e) => {return Err(e);},
                Ok(s) => s
            }
        }))?)

    }

    pub async fn test(&self) -> Result<String, Error> {
        // SETTIN THE STATE WITH FAKE PROFILES -- FOR TESTING
        let users = self.create_fake_profiles().await?; //
        self.state.set(Field::Users(Some(users))).await?;

        let count = self.state.get_or_default::<u64>(&Field::Test(None)).await?;
        self.state.set(Field::Test(Some(count+1))).await?;
        Ok(format!("{{\"count\": {}}}", count/10))
    }

    /*      Bitcoin Home        */
    pub async fn bitcoin_home(&self) -> Result<String, Error> {
        let profile_pfp = self.state.get_or_default::<Profile>(&Field::Profile(None)).await?.pfp_path;
        let balance = sats_to_btc(self.state.get_or_default::<Sats>(&Field::Balance(None)).await?);
        let transactions: BTreeMap<Txid, Transaction> = self.state.get_or_default(&Field::Transactions(None)).await?;
        let price = self.state.get_or_default::<Usd>(&Field::Price(None)).await?;
        let mut transactions = transactions.into_iter().collect::<Vec<(Txid, Transaction)>>();
        transactions.sort_by_key(|(_, tx)| tx.confirmation_time.as_ref().unwrap_or(&tx.time_created).clone());
        transactions.reverse();
        Ok(serde_json::to_string(&json!({
            "balance_btc": format_btc(balance),
            "balance_usd": format_usd(balance*price),
            "profile_picture": profile_pfp,
            "transactions": transactions.into_iter().map(|(txid, tx)| {
                let date_time = format_datetime(tx.confirmation_time.as_ref());
                ShorthandTransaction{
                    is_withdraw: tx.is_withdraw,
                    datetime: date_time,
                    amount: format_usd(sats_to_btc(tx.sats)*tx.price),
                    txid: txid.to_string()
                }
            }).collect::<Vec<ShorthandTransaction>>()
        }))?)
    }

    pub async fn view_transaction(&self, txid: String) -> Result<String, Error> {
        let txid = Txid::from_str(&txid).map_err(|e| Error::err("Txid::from_str", &e.to_string()))?;
        let transactions = self.state.get_or_default::<BTreeMap<Txid, Transaction>>(&Field::Transactions(None)).await?;
        let tx = transactions.get(&txid).ok_or(Error::err("view_transaction", "No transaction found for txid"))?;
        let dt = tx.confirmation_time.as_ref();

        let btc = sats_to_btc(tx.sats);
        let usd = btc * tx.price;
        let fee_usd = sats_to_btc(tx.fee)*tx.price;

        Ok(serde_json::to_string(&json!({
            "is_withdraw": tx.is_withdraw,
            "date": dt.map(|dt| dt.format("%m/%d/%Y").to_string()).unwrap_or("Pending".to_string()),
            "time": dt.map(|dt| dt.format("%l:%M %p").to_string()).unwrap_or("Pending".to_string()),
            "address": format_adr(&tx.address),
            "amount_btc": format_btc(btc),
            "amount_usd": format_usd(usd),
            "price": format_usd(tx.price),
            "fee": if tx.is_withdraw {Some(format_usd(fee_usd))} else {None},
            "total": if tx.is_withdraw {Some(format_usd(fee_usd+usd))} else {None}
        }))?)
    }

    pub async fn receive(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&json!({
            "address": call_thread(Threads::Wallet(WalletMethod::GetNewAddress)).await?,
        }))?)
    }

    pub async fn send(&self, address: &str) -> Result<String, Error> {
        let valid_address = if address.is_empty() {true} else {Address::from_str(address).ok().map(|a| a.require_network(Network::Bitcoin).is_ok()).unwrap_or(false)};
        Ok(serde_json::to_string(&json!({
            "valid_address": valid_address,
        }))?)
    }

    pub async fn amount(&self, amount: String) -> Result<String, Error> {
        let price = self.state.get_or_default::<Usd>(&Field::Price(None)).await?;
        let balance = sats_to_btc(self.state.get_or_default::<Sats>(&Field::Balance(None)).await?) * price;

        let amount_usd = amount.parse::<Usd>().unwrap_or(0.0);
        let amount_btc = amount_usd / price;
        let amount_sats = (amount_btc * SATS as Btc) as Sats;

        let min: Usd = 0.30;
        let max = balance - min;

        let err = if amount_usd != 0.0 {
            if max <= 0.0 {
                Some("You don't have enough bitcoin".to_string())
            } else if amount_usd < min {
                Some(format!("${:.2} minimum", min))
            } else if amount_usd > max {
                Some(format!("${:.2} maximum", max))
            } else {
                None
            }
        } else {None};


        Ok(serde_json::to_string(&json!({
            "amount_btc": format_btc(amount_btc),
            "amount_sats": amount_sats,
            "err": err,
        }))?)
    }

    pub async fn speed(&self, amount: Sats) -> Result<String, Error> {
        let price = self.state.get_or_default::<Usd>(&Field::Price(None)).await?;
        let fees = serde_json::from_str::<(Sats, Sats)>(&call_thread(Threads::Wallet(WalletMethod::GetFees(amount))).await?)?;
        Ok(serde_json::to_string(&json!({
            "priority_f": format_usd((fees.0 as Btc / SATS as Btc) * price),
            "priority": fees.0,
            "standard_f": format_usd((fees.1 as Btc / SATS as Btc) * price),
            "standard": fees.1
        }))?)
    }

    pub async fn confirm(&self, address: String, amount_sats: Sats, fee: Sats) -> Result<String, Error> {
        let price = self.state.get_or_default::<Usd>(&Field::Price(None)).await?;
        let (btx, tx) = serde_json::from_str::<(bdk::bitcoin::Transaction, Transaction)>(
            &call_thread(Threads::Wallet(WalletMethod::BuildTransaction(
                address, amount_sats, fee, price
            ))).await?
        )?;
        let btc = sats_to_btc(tx.sats);
        let usd = btc * tx.price;
        let fee_usd = sats_to_btc(tx.fee)*tx.price;
        Ok(serde_json::to_string(&json!({
            "raw_tx": serde_json::to_string(&btx)?,
            "address_cut": format_adr(&tx.address),
            "address_whole": tx.address,
            "amount_btc": format_btc(btc),
            "amount_usd": format_usd(usd),
            "amount": format_usd(usd).replace("$", ""),
            "fee_usd": format_usd(fee_usd),
            "total": format_usd(usd + fee_usd),
        }))?)
    }

    pub async fn success(&self, raw_tx: String) -> Result<String, Error> {
        let tx: bdk::bitcoin::Transaction = serde_json::from_str(&raw_tx)?;
        call_thread(Threads::Wallet(WalletMethod::BroadcastTransaction(tx))).await?;
        Ok("{}".to_string())
    }

    pub async fn scan_qr(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&json!({}))?)
    }

    /*      Messages Home        */

    pub async fn my_profile(&self, init: bool) -> Result<String, Error> {
        let profile = self.state.get_or_default::<Profile>(&Field::Profile(None)).await?;
        let address = if init {
            let adr = call_thread(Threads::Wallet(WalletMethod::GetNewAddress)).await?;
            self.state.set(Field::ProfileAddress(Some(adr.clone()))).await?;
            adr
        } else {self.state.get_or_default::<String>(&Field::ProfileAddress(None)).await?};

        Ok(serde_json::to_string(&json!({
            "name": profile.name,
            "abt_me": profile.abt_me,
            "did": profile.did,
            "profile_picture": profile.pfp_path,
            "address": address,
        }))?)
    }

    pub async fn user_profile(&self, init: bool, profile: DartProfile) -> Result<String, Error> {
        log::info!("USER PROFILE {:?}", profile.clone());
        let address = if init {
            call_thread(Threads::Wallet(WalletMethod::GetNewAddress)).await? // Needs to get for users address
        } else {String::new()};

        Ok(serde_json::to_string(&json!({
            "name": profile.name,
            "abt_me": profile.abt_me,
            "did": profile.did,
            "profile_picture": profile.pfp_path,
            "address": address,
        }))?)
    }

    pub async fn messages_home(&self) -> Result<String, Error> {
        let profile_pfp = self.state.get_or_default::<Profile>(&Field::Profile(None)).await?.pfp_path;
        let conversations = self.state.get_or_default::<Vec<Conversation>>(&Field::Conversations(None)).await?;
        let conversations: BTreeMap<String, Conversation> = conversations.into_iter().map(|conversation| (conversation.room_id.clone(), conversation)).collect();

        Ok(serde_json::to_string(&json!({
            "profile_picture": profile_pfp,
            "conversations": conversations.into_iter().map(|(room_id, cv)| {
                let is_group = cv.members.len() > 1;
                let room_name = if is_group { "Group Message".to_string() } else { cv.members[0].name.clone() };
                let photo = if !is_group && !cv.members.is_empty() { cv.members[0].pfp_path.clone() } else { None };

                let subtext = if is_group {
                    cv.members.iter().map(|profile| profile.name.as_str()).collect::<Vec<_>>().join(", ")
                } else if let Some(first_message) = cv.messages.first() {
                    if !first_message.is_incoming { format!("You: {}", first_message.message) } else { first_message.message.to_string() }
                } else {
                    "No messages yet".to_string()
                };

                ShorthandConversation { room_name, photo, subtext, room_id, is_group }
            }).collect::<Vec<ShorthandConversation>>()
        }))?)

    }

    pub async fn conversation_info(&self, record: String) -> Result<String, Error> {
        let conversations = self.state.get_or_default::<Vec<Conversation>>(&Field::Conversations(None)).await?;
        let conversations: BTreeMap<String, Conversation> = conversations.into_iter().map(|conversation| (conversation.room_id.clone(), conversation)).collect();
        let members = conversations.get(&record).ok_or(Error::err("current_conversation", "No conversation found for room record"))?.clone().members;

        Ok(serde_json::to_string(&json!({
            "members": members.into_iter().map(|m| DartProfile {
                name: m.name,
                did: m.did.to_string(),
                abt_me: m.abt_me,
                pfp_path: m.pfp_path
            }).collect::<Vec<DartProfile>>(),
        }))?)
    }

    pub async fn current_conversation(&self, record: String, members: Vec<DartProfile>) -> Result<String, Error> {
        let mut vec_conversations = self.state.get_or_default::<Vec<Conversation>>(&Field::Conversations(None)).await?;
        //let mut new_message = self.state.get_or_default::<Message>(&Field::NewMessage(None)).await?;
        // IF there is no record, create a new conversation
        let record = if record.is_empty() {

            // Collect the members as Web5 Profiles from DartProfiles
            log::info!("ALL THE ----------- MEMBERS::: {:?}", members.clone());
            let chosen_members = members.into_iter().map(|dart_profile| Profile {
                name: dart_profile.name,
                did: Did::from_str(dart_profile.did.as_str()).unwrap(),
                abt_me: dart_profile.abt_me,
                pfp_path: dart_profile.pfp_path,
            }).collect::<Vec<Profile>>();

            // Create a record
            let record: String = thread_rng().sample_iter(&Alphanumeric).take(5).map(char::from).collect();

            // Create a new conversation and push it to the state
            let new_conversation = Conversation::new(chosen_members, Vec::new(), record.clone());
            vec_conversations.push(new_conversation.clone());
            self.state.set(Field::Conversations(Some(vec_conversations.clone()))).await?;

            // Return the new record
            record
        } else { record };
        // Find my conversation based of the record
        let mut map_conversations: BTreeMap<String, Conversation> = vec_conversations.into_iter().map(|conversation| (conversation.room_id.clone(), conversation)).collect();
        let this_conversation = map_conversations.get_mut(record.as_str()).ok_or(Error::err("current_conversation", format!("No conversation found for room record {:?}", &record).as_str()))?;

        // IF there is a new message, push it to the messages
        // if new_message.is_some() {

        //     // Get my profile and create a new message
        //     let profile = self.state.get::<Profile>(&Field::Profile(None)).await?;
        //     let new_message: Message = create_message(new_message.clone().unwrap_or("Message failed to send".to_string()), profile).await;

        //     // Add the message to the conversaion and update the conversations list
        //     this_conversation.messages.push(new_message);
        //     let updated_conversations = map_conversations.iter().map(|(_, conversation)| conversation.clone()).collect::<Vec<Conversation>>();
        //     self.state.set(Field::Conversations(Some(updated_conversations))).await?;
        // } 

        // Resetting new message to null
        let new_message: Option<String> = None;
        
        // Have to remap the conversations cause creating a new one turns the map immutable. 
        // RUST IS DUMB DUMB
        let this_conversation = map_conversations.get(&record).ok_or(Error::err("current_conversation", "No conversation found for room record"))?;

        // Get the members and messages out of this_conversation
        let members = this_conversation.members.clone();
        let messages = this_conversation.messages.clone();
        log::info!("ALL  MEMBERS::: {:?}", members.clone());
    
        Ok(serde_json::to_string(&json!({
            "room_id": record,
            "room_name": if members.len() > 1 { "Group Message".to_string() } else { members[0].name.clone() },
            "is_group": members.len() > 1,
            "members": members.into_iter().map(|m| Profile { name: m.name, did: m.did, abt_me: m.abt_me, pfp_path: m.pfp_path }).collect::<Vec<Profile>>(),

            "messages": Some(messages.into_iter().map(|m| Message {
                sender: Profile { name: m.sender.name, did: m.sender.did, abt_me: m.sender.abt_me, pfp_path: m.sender.pfp_path },
                message: m.message,
                date: "11/11/11".to_string(),
                time: "11:11 PM".to_string(),
                is_incoming: m.is_incoming,
            }).collect::<Vec<Message>>()),
            "new_message": new_message,
        }))?)
    }

    pub async fn choose_recipient(&self) -> Result<String, Error> {
        let users = self.state.get_or_default::<Vec<Profile>>(&Field::Users(None)).await?;
        Ok(serde_json::to_string(&json!({
            "users": users.into_iter().map(|profile| {
                DartProfile{
                    name: profile.name,
                    did: profile.did.to_string(),
                    abt_me: profile.abt_me,
                    pfp_path: profile.pfp_path,
                }
            }).collect::<Vec<DartProfile>>()
        }))?)
    }
    

    // USED FOR TESTING, CAN BE DELETED //
    fn generate_random_did() -> Did {
        let random_part: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        Did::from_str(format!("did:dht:{}", random_part).as_str()).unwrap()
    }

    // USED FOR TESTING, CAN BE DELETED //
    pub async fn create_fake_profiles(&self) -> Result<Vec<Profile>, Error> {
        let fake_profiles = vec![
            Profile {
                name: "Avery Bloom".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Gardener and plant enthusiast, finding beauty in every green corner.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Casey Ray".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Photographer capturing moments that tell stories.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Riley Sage".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Coder by day, gamer by night. Always up for a good challenge.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Jordan Quinn".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Traveler with a knack for storytelling. Exploring one place at a time.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Taylor Vale".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("DIY enthusiast with a passion for building and fixing things.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Ike Stone".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Architect turned urban explorer. I build during the week and explore on weekends.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Derrik North".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Ex-military, now a survival instructor. Always ready for an adventure.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Jordan Ember".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Writer and history buff. Stories are everywhere if you look close enough.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Tara Ridge".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Hiker, geologist, and rock collector. The world is my lab.".to_string()),
                pfp_path: None,
            },
            Profile {
                name: "Quentin Vale".to_string(),
                did: Self::generate_random_did(),
                abt_me: Some("Engineer and tinkerer. If it’s broken, I’ll fix it. If it works, I’ll make it better.".to_string()),
                pfp_path: None,
            },
        ];

        Ok(fake_profiles)
    }

}

// USED FOR TESTING, CAN BE DELETED //
pub async fn create_message(message: String, profile: Profile) -> Message {
    let now: DateTime = DateTime::now();
    let formatted_time = now.format("%I:%M%p").to_string();
    Message::new(profile, message, now.date.to_string(), formatted_time, false)
}

pub fn format_usd(price: Usd) -> String {
    let price = (price * 100.0).round() / 100.0;
    let whole_part = price.trunc() as i64;
    let decimal_part = ((price.fract() * 100.0) as u64) % 100;
    format!("${}.{:02}", whole_part.to_formatted_string(&Locale::en), decimal_part)
}

pub fn format_btc(btc: Btc) -> String {format!("{:.8} BTC", btc)}

pub fn sats_to_btc(sats: Sats) -> Btc {sats as Btc / SATS as Btc}

pub fn format_adr(adr: &str) -> String {
    if adr.starts_with("The ") { return "Redeposit".to_string();}
    format!("{}...{}", &adr[..9], &adr[adr.len()-3..])
}

pub fn format_datetime(date: Option<&DateTime>) -> String {
    if let Some(dt) = date {
        let duration = DateTime::now().date.signed_duration_since(dt.date);
        if duration.num_days() == 0 {
            dt.format("%l:%M %p").to_string()
        } else if duration.num_days() == 1 {
            "Yesterday".to_string()
        } else if duration.num_days() <= 365 {
            dt.format("%B %e").to_string()
        } else {
            dt.format("%m/%d/%Y").to_string()
        }
    } else {"Pending".to_string()}
}
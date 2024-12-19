use super::Error;

use super::price::PriceGetter;
use super::state::{State};
use super::structs::{Callback, DateTime, Request};
use super::state::Field;
use super::pub_structs::{Sats, Usd};

use std::collections::BTreeMap;
use std::convert::TryInto;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use bdk::{TransactionDetails, KeychainKind, SyncOptions, SignOptions};
use bdk::bitcoin::psbt::PartiallySignedTransaction as PSBT;
use bdk::blockchain::electrum::ElectrumBlockchain;
use bdk::bitcoin::blockdata::script::Script;
use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::template::DescriptorTemplate;
use bdk::bitcoin::{Network, Address};
use bdk::bitcoin::hash_types::Txid;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::blockchain::Blockchain;
use bdk::wallet::AddressIndex;
use bdk::Wallet as BDKWallet;
use bdk::template::Bip86;

use serde::{Serialize, Deserialize};
use secp256k1::rand::RngCore;
use secp256k1::rand;
use tokio::sync::Mutex;

const CLIENT_URI: &str = "ssl://electrum.blockstream.info:50002";
const DUMMY_ADDRESS: &str = "bc1qxma2dwmutht4vxyl6u395slew5ectfpn35ug9l";

pub type Transactions = BTreeMap<Txid, Transaction>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Seed{
    pub inner: Vec<u8>
}

impl Seed {
    pub fn new() -> Self {
        let mut inner: [u8; 64] = [0; 64];
        rand::thread_rng().fill_bytes(&mut inner);
        Seed{inner: inner.to_vec()}
    }

    pub fn get(&self) -> [u8; 64] {self.inner.clone().try_into().unwrap()}
}

impl Default for Seed {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DescriptorSet {
    pub external: String,
    pub internal: String
}

impl DescriptorSet {
    pub fn from_seed(seed: &Seed) -> Result<Self, Error> {
        let xpriv = ExtendedPrivKey::new_master(Network::Bitcoin, &seed.get())?;
        let ex_desc = Bip86(xpriv, KeychainKind::External).build(Network::Bitcoin)?;
        let external = ex_desc.0.to_string_with_secret(&ex_desc.1);
        let in_desc = Bip86(xpriv, KeychainKind::Internal).build(Network::Bitcoin)?;
        let internal = in_desc.0.to_string_with_secret(&in_desc.1);
        Ok(DescriptorSet{external, internal})
    }
}

//TODO: build a global Sats to btc and Sats to usd function
//TODO: No Usd Values only Sats
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Transaction {
    pub sats: Sats,
    pub fee: Sats,
    pub price: Usd,
    pub address: String,
    pub is_withdraw: bool,
    pub confirmation_time: Option<DateTime>,
    pub time_created: DateTime,
}

impl Transaction {
    pub fn from_details(details: TransactionDetails, price: Usd, is_mine: impl Fn(&Script) -> bool) -> Result<Self, Error> {
        let ec = "Transaction::from_details";
        let fee = details.fee.ok_or(Error::bad_request(ec, "Missing Fee"))?;
        let is_withdraw = details.sent > 0;
        let sats = if details.sent >= details.received {
            details.sent - details.received
        } else {
            details.received - details.sent
        };
        let sats = if is_withdraw {sats-fee} else {sats};
        let details_tx = details.transaction.ok_or(Error::bad_request(ec, "Missing Transaction"))?;
        let address = if is_withdraw {
            details_tx.output.clone().into_iter().find_map(|txout| {
                if !is_mine(txout.script_pubkey.as_script()) {
                    Some(Address::from_script(txout.script_pubkey.as_script(), Network::Bitcoin).ok()?.to_string())
                } else {None}
            }).unwrap_or("The Bitcoin you are trying to send will be redeposited to your account.".to_string())
        } else {
            let txout = details_tx.output.clone().into_iter().find(|txout| is_mine(txout.script_pubkey.as_script())).ok_or(Error::bad_request(ec, "No Output is_mine"))?;
            Address::from_script(txout.script_pubkey.as_script(), Network::Bitcoin)?.to_string()
        };
        let confirmation_time = details.confirmation_time.map(|ct|
            DateTime::from_timestamp(ct.timestamp)
        ).transpose()?;
        Ok(Transaction{sats, fee, price, address, is_withdraw, confirmation_time, time_created: DateTime::now()})
    }
}

pub struct Wallet{
    inner: Arc<Mutex<BDKWallet<SqliteDatabase>>>,
    descriptors: DescriptorSet,
    path: PathBuf
}

impl Wallet {
    #[async_backtrace::framed]
    pub async fn new(
        _callback: Callback,
        path: PathBuf,
    ) -> Result<Self, Error> {
        //let seed: Seed = if let Some(seed) = storage.get("legacy_seed").await? {
        //    serde_json::from_str(&seed)?
        //} else {
        //    let seed = Seed::new();
        //    storage.set("legacy_seed", &serde_json::to_string(&seed)?).await?;
        //    seed
        //};
        //Hard coded for testing
        let seed: Seed = Seed{inner: vec![175, 178, 194, 229, 165, 10, 1, 80, 224, 239, 231, 107, 145, 96, 212, 195, 10, 78, 64, 17, 241, 77, 229, 246, 109, 226, 14, 83, 139, 28, 232, 220, 5, 150, 79, 185, 67, 31, 247, 41, 150, 36, 77, 199, 67, 47, 157, 15, 61, 142, 5, 244, 245, 137, 198, 34, 174, 221, 63, 134, 129, 165, 25, 7]};
        let descriptors = DescriptorSet::from_seed(&seed)?;
        Ok(Wallet{
            inner: Self::inner_wallet(&descriptors, path.clone())?,
            descriptors,
            path
        })
    }

    fn inner_wallet(
        descriptors: &DescriptorSet,
        path: PathBuf
    ) -> Result<Arc<Mutex<BDKWallet<SqliteDatabase>>>, Error> {
        std::fs::create_dir_all(path.clone())?;
        Ok(Arc::new(Mutex::new(BDKWallet::new(
            &descriptors.external,
            Some(&descriptors.internal),
            Network::Bitcoin,
            SqliteDatabase::new(path.join("bdk.db"))
        )?)))
    }

    #[async_backtrace::framed]
    pub async fn get_new_address(&self) -> Result<String, Error> {
        Ok(self.inner.lock().await.get_address(AddressIndex::New)?.address.to_string())
    }

    #[async_backtrace::framed]
    pub async fn get_fees(&self, amount: Sats) -> Result<(Sats, Sats), Error> {
        let client = Self::get_blockchain()?;
        let one = Request::process_result(client.estimate_fee(1))?.as_sat_per_vb() as Sats;
        let vb = self.tx_builder(DUMMY_ADDRESS, amount, one * 200).await?.0.extract_tx().vsize() as u64;
        Ok((
            (one * 2) * vb,
            one * vb
        ))
    }

    #[async_backtrace::framed]
    async fn tx_builder(&self, address: &str, amount: Sats, fee: Sats) -> Result<(PSBT, bdk::TransactionDetails), Error> {
        let inner = self.inner.lock().await;
        let mut builder = inner.build_tx();
        let address = Address::from_str(address)?.require_network(Network::Bitcoin)?;
        builder.add_recipient(address.script_pubkey(), amount);
        builder.fee_absolute(fee);
        Ok(builder.finish()?)
    }

    #[async_backtrace::framed]
    pub async fn build_transaction(&self, address: &str, amount: Sats, fee: Sats, price: Usd) -> Result<(bdk::bitcoin::Transaction, Transaction), Error> {
        log::info!("fee bt: {}", fee);
        let (mut psbt, mut tx_details) = self.tx_builder(address, amount, fee).await?;

        let inner = self.inner.lock().await;
        inner.sign(&mut psbt, SignOptions::default())?;

        let tx = psbt.clone().extract_tx();
        tx_details.transaction = Some(tx.clone());

        let transaction = Transaction::from_details(tx_details, price, |s: &Script| -> bool {inner.is_mine(s).unwrap_or_default()})?;
        Ok((tx, transaction))
    }


    #[async_backtrace::framed]
    pub async fn broadcast_transaction(&self, tx: &bdk::bitcoin::Transaction) -> Result<(), Error> {
        Request::process_result(Self::get_blockchain()?.broadcast(tx))
    }

    fn get_blockchain() -> Result<ElectrumBlockchain, Error> {
        Ok(ElectrumBlockchain::from(Request::process_result(Client::new(CLIENT_URI))?))
    }
    
    #[async_backtrace::framed]
    pub async fn sync(&self) -> Result<(), Error> {
        let blockchain = Self::get_blockchain().map_err(|e| Error::bad_request("Failed to get blockchain", &format!("{}", e)))?;
        Request::process_result(self.inner.lock().await.sync(&blockchain, SyncOptions::default()))?;
        Ok(())
    }

    #[async_backtrace::framed]
    pub async fn refresh_state(&self, state: &State) -> Result<(), Error> {
        let inner = self.inner.lock().await;
        //Transactions
        let mut txs = state.get_or_default::<Transactions>(&Field::Transactions(None)).await?;
        for tx in inner.list_transactions(true)? {
            if let Some(transaction) = txs.get(&tx.txid) {
                if transaction.confirmation_time.is_some() {continue;}
                if tx.confirmation_time.is_none() {continue;}
            }

            let price = if let Some(ct) = tx.confirmation_time.as_ref() {
                PriceGetter::get(Some(&DateTime::from_timestamp(ct.timestamp)?)).await?
            } else {
                state.get_or_default::<Usd>(&Field::Price(None)).await?
            };
            txs.insert(tx.txid, Transaction::from_details(tx, price, |s: &Script| -> bool {inner.is_mine(s).unwrap_or_default()})?);
        }
        let balance = txs.values().map(|tx| if tx.is_withdraw {-(tx.sats as i64)} else {tx.sats as i64}).sum::<i64>() as u64;
        state.set(Field::Transactions(Some(txs))).await?;
        //Balance
        state.set(Field::Balance(Some(balance))).await?;
        Ok(())
    }
}

impl Clone for Wallet {
    fn clone(&self) -> Self {
        Wallet{
            inner: Wallet::inner_wallet(&self.descriptors, self.path.clone()).unwrap(),
            descriptors: self.descriptors.clone(),
            path: self.path.clone()
        }
    }
}

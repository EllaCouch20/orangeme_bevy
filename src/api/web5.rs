use super::Error;

use super::pub_structs::DartMethod;
use super::state::{State, Field};
use super::structs::{Callback, Request};

use std::path::PathBuf;
use std::str::FromStr;

use simple_database::{KeyValueStore, SqliteStore};
use simple_database::database::{FiltersBuilder, IndexBuilder, Filter};

use web5_rust::{
    ChannelPermissionOptions,
    PermissionOptions,
    ChannelProtocol,
    DwnResponse,
    Protocol,
    Packet,
};
use web5_rust::dids::{DidResolver, DefaultDidResolver, DidDocument, DhtDocument, Identity, Did};
use web5_rust::{Wallet, Record, Agent, DefaultRouter};
use web5_rust::json_rpc::JsonRpcClient;
use web5_rust::traits::Client;

use schemars::schema::Schema;
use schemars::schema_for;
use schemars::JsonSchema;

use serde::{Serialize, Deserialize};

const DUMMY_DID: &str = "did:dht:0000000000000000000000000000000000000000000000000000";

pub struct Protocols {}
impl Protocols {
    pub fn profile() -> Protocol {
         Protocol::new(
            "Profile",
            true,
            PermissionOptions::new(true, true, false, None),
            Some(serde_json::to_string(&schema_for!(Profile)).unwrap()),
            None
        ).unwrap()
    }

    pub fn shorthand_conversations() -> Protocol {
        Protocol::new(
            "Shorthand_Conversation",
            true,
            PermissionOptions::new(true, true, false, None),
            Some(serde_json::to_string(&schema_for!(ShorthandConversation)).unwrap()),
            None
        ).unwrap()
    }

    pub fn conversation() -> Protocol {
        Protocol::new(
            "Conversation",
            true,
            PermissionOptions::new(true, true, false, None),
            Some(serde_json::to_string(&schema_for!(Conversation)).unwrap()),
            None
        ).unwrap()
    }

    pub fn message() -> Protocol {
        Protocol::new(
            "Single_Message",
            true,
            PermissionOptions::new(true, true, false, None),
            Some(serde_json::to_string(&schema_for!(Message)).unwrap()),
            None
        ).unwrap()
    }

    pub fn messages_protocol() -> Protocol {
         Protocol::new(
            "Message",
            true,
            PermissionOptions::new(true, true, false, None),
            Some(serde_json::to_string(&Schema::Bool(true)).unwrap()),
            None
        ).unwrap()
    }

    pub fn rooms_protocol() -> Protocol {
        Protocol::new(
            "Room",
            false,
            PermissionOptions::new(true, true, false, Some(
                ChannelPermissionOptions::new(true, true, true)
            )),
            Some(serde_json::to_string(&Schema::Bool(true)).unwrap()),
            Some(ChannelProtocol::new(
                Some(vec![&Self::messages_protocol()])
            ))
        ).unwrap()
    }

    pub fn get() -> Vec<Protocol> {
        vec![Self::messages_protocol(), Self::shorthand_conversations(), Self::conversation(), Self::message(), Self::rooms_protocol(), Self::profile()]
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Profile {
    pub name: String,
    pub did: Did,
    pub pfp_path: Option<String>,
    pub abt_me: Option<String>,
}

impl Profile {
    pub fn new(
        name: String,
        did: Did,
        pfp_path: Option<String>,
        abt_me: Option<String>,
    ) -> Self {
        Profile{name, did, pfp_path, abt_me}
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile::new("Default Name".to_string(), Did::from_str(DUMMY_DID).unwrap(), None, None)
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct ShorthandConversation {
    pub room_name: String,
    pub photo: Option<String>,
    pub subtext: String,
    pub is_group: bool,
    pub room_id: String,
}

impl ShorthandConversation {
    pub fn new(
        room_name: String,
        photo: Option<String>,
        subtext: String,
        is_group: bool,
        room_id: String
    ) -> Self {
        ShorthandConversation{room_name, photo, subtext, is_group, room_id}
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]//Ord, PartialOrd, Eq, PartialEq
pub struct Conversation {
    pub members: Vec<Profile>,
    pub messages: Vec<Message>,
    pub room_id: String,
}

impl Conversation {
    pub fn new(
        members: Vec<Profile>,
        messages: Vec<Message>,
        room_id: String,
    ) -> Self {
        Conversation{members, messages, room_id}
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Message {
    pub sender: Profile,
    pub message: String,
    pub date: String,
    pub time: String,
    pub is_incoming: bool,
}

impl Message {
    pub fn new(
        sender: Profile,
        message: String,
        date: String,
        time: String,
        is_incoming: bool,
    ) -> Self {
        Message{sender, message, date, time, is_incoming}
    }
}

#[derive(Clone, Debug)]
pub struct MessagingAgent {
    agent: Agent
}

impl MessagingAgent {
    #[async_backtrace::framed]
    pub async fn new(callback: Callback, path: PathBuf) -> Result<Self, Error> {
        let callback = callback.lock().await;
        let (doc, id) = if let Some(i) = callback(DartMethod::StorageGet("identity".to_string())).await {
            serde_json::from_str::<(DhtDocument, Identity)>(&i)?
        } else {
            let tup = DhtDocument::default(vec!["did:dht:fxaigdryri3os713aaepighxf6sm9h5xouwqfpinh9izwro3mbky".to_string()])?;
            callback(DartMethod::StorageSet("identity".to_string(), serde_json::to_string(&tup)?)).await;
            tup
        };

        Request::repeat(|| {
            let key = id.did_key.clone();
            let doc = doc.clone();
            Box::pin(async move {doc.publish(&key).await})
        }).await?;

        let did_resolver = Box::new(IntervalDidResolver::new::<SqliteStore>(Some(path.join("DefaultDidResolver"))).await?);
        let client = Box::new(IntervalClient::new());
        let router = Box::new(DefaultRouter::new(did_resolver.clone(), Some(client)));
        Ok(MessagingAgent{
            agent: Agent::new::<SqliteStore>(
                Wallet::new(id, did_resolver.clone(), Some(router.clone()))
                    .get_agent_key(&Protocols::rooms_protocol()).await?,
                Protocols::get(), Some(path), Some(did_resolver), Some(router)
            ).await?
        })
    }
    #[async_backtrace::framed]
    pub async fn set_profile(&self, profile: Profile) -> Result<(), Error> {
        let index = IndexBuilder::build(vec![("type", "profile")]);
        let record = Record::new(None, &Protocols::profile(), serde_json::to_vec(&profile)?);
        self.agent.public_update(record, index, None).await?;
        Ok(())
    }
    #[async_backtrace::framed]
    pub async fn init_profile(&self, state: &State) -> Result<(), Error> {
        let tenant = self.agent.tenant().clone();
        if let Some(p) = self.agent.public_read(FiltersBuilder::build(vec![
            ("author", Filter::equal(tenant.to_string())),
            ("type", Filter::equal("profile"))]
        ), None, None).await?.first() {
            let profile = serde_json::from_slice::<Profile>(&p.1.payload)?;
            state.set(Field::Profile(Some(profile))).await?;
        } else {
            let index = IndexBuilder::build(vec![("type", "profile")]);
            let profile = Profile::new("Default Name".to_string(), tenant, None, None);
            let record = Record::new(None, &Protocols::profile(), serde_json::to_vec(&profile)?);
            self.agent.public_create(record, index, None).await?;
            state.set(Field::Profile(Some(profile))).await?;
        }
        Ok(())
    }

    #[async_backtrace::framed]
    pub async fn sync(&self) -> Result<(), Error> {
        self.agent.scan().await?;
        Ok(())
    }

    #[async_backtrace::framed]
    pub async fn refresh_state(&self, state: &State) -> Result<(), Error> {
        self.init_profile(state).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IntervalClient {
    client: JsonRpcClient
}

impl IntervalClient {
    pub fn new() -> Self {
        IntervalClient{client: JsonRpcClient{}}
    }
}

impl Default for IntervalClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Client for IntervalClient {
    async fn send_packet(
        &self,
        p: Packet,
        url: url::Url
    ) -> Result<DwnResponse, web5_rust::Error> {
        Request::repeat(|| {
            let client = self.client.clone();
            let p = p.clone();
            let url = url.clone();
            Box::pin(async move {client.send_packet(p, url).await})
        }).await
    }
}

#[derive(Debug, Clone)]
pub struct IntervalDidResolver {
    resolver: DefaultDidResolver
}

impl IntervalDidResolver {
    #[async_backtrace::framed]
    pub async fn new<KVS: KeyValueStore + 'static>(path: Option<PathBuf>) -> Result<Self, Error> {
        Ok(IntervalDidResolver{
            resolver: DefaultDidResolver::new::<KVS>(path).await?
        })
    }
}

#[async_trait::async_trait]
impl DidResolver for IntervalDidResolver {
    #[async_backtrace::framed]
    async fn resolve(&self, did: &Did) -> Result<Option<Box<dyn DidDocument>>, web5_rust::Error> {
        Request::repeat(|| {
            let resolver = self.resolver.clone();
            let did = did.clone();
            Box::pin(async move {resolver.resolve(&did).await})
        }).await
    }
}

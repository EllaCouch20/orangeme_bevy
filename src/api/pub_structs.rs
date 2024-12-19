use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Platform {
    Mac,
    Linux,
    Windows,
    IOS,
    Android,
    Fuchsia
}

impl Platform {
    #[flutter_rust_bridge::frb(sync)]
    pub fn is_desktop(&self) -> bool {
        matches!(self, Platform::Mac | Platform::Linux | Platform::Windows)
    }
}

#[derive(Debug)]
pub enum PageName {
    BitcoinHome,
    ViewTransaction(String),
    Receive,
    Send(String),
    Amount(String),
    Speed(Sats),
    Confirm(String, Sats, Sats),
    Success(String),
    MyProfile(bool),
    UserProfile(bool, DartProfile),
    MessagesHome,
    ChooseRecipient,
    CurrentConversation(String, Vec<DartProfile>),
    ConversationInfo(String),
    Test(String),
    Scan,
}

#[derive(Debug)]
pub enum KeyPress {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Reset,
    Decimal,
    Backspace,
}

pub const SATS: u64 = 100_000_000;
pub type Sats = u64;
pub type Usd = f64;
pub type Btc = f64;

pub enum DartMethod {
    StorageSet(String, String),
    StorageGet(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShorthandTransaction {
    pub is_withdraw: bool,
    pub datetime: String,
    pub amount: String,
    pub txid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShorthandConversation {
    pub room_name: String,
    pub photo: Option<String>,
    pub subtext: String,
    pub is_group: bool,
    pub room_id: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Conversation {
//     pub members: Vec<Profile>,
//     pub messages: Vec<Message>,
//     pub room_id: String,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub sender: DartProfile,
    pub message: String,
    pub date: String,
    pub time: String,
    pub is_incoming: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DartProfile {
    pub name: String,
    pub did: String,
    pub pfp_path: Option<String>,
    pub abt_me: Option<String>,
}

pub fn load_structs(_s: ShorthandTransaction, _sc: ShorthandConversation, _m: Message, _p: DartProfile, _dm: DartMethod, _kp: KeyPress) {}
pub fn load_structs2(_pl: Platform, _pn: PageName) {}

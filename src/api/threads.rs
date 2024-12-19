use super::Error;

use super::pub_structs::{Sats, Usd};
use super::state::{State, Field};
use super::web5::MessagingAgent;
use super::price::PriceGetter;
use super::structs::{Callback, Request};
use super::wallet::Wallet;

use std::sync::LazyLock;
use std::path::PathBuf;
use std::time::Duration;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{oneshot, Mutex, mpsc};
use tokio::time;

static THREAD_CHANNELS: LazyLock<Mutex<(Option<WalletSender>, Option<String>)>> = LazyLock::new(|| Mutex::new((None, None)));

pub type ChannelType = Result<String, Error>;
pub type WalletChannel = (oneshot::Sender<ChannelType>, WalletMethod);
pub type WalletSender = Sender<WalletChannel>;
pub type WalletReceiver = Receiver<WalletChannel>;

pub enum Threads {
    Wallet(WalletMethod)
}

#[derive(Debug)]
pub enum WalletMethod {
    GetNewAddress,
    GetFees(Sats),
    BuildTransaction(String, Sats, Sats, Usd),
    BroadcastTransaction(bdk::bitcoin::Transaction)
}

#[async_backtrace::framed]
pub async fn start_threads(state: State, callback: Callback, path: PathBuf) -> Result<(), Error> {
    //Init channels
    let mut threads = THREAD_CHANNELS.lock().await;
    let (w_tx, w_rx) = mpsc::channel::<WalletChannel>(100);
    threads.0 = Some(w_tx);
    drop(threads);

    let err = tokio::select!(
        Err(e) = spawn(internet_thread(state.clone())) => e,
        Err(e) = spawn(price_thread(state.clone())) => e,
        Err(e) = spawn(wallet_thread(state.clone(), callback.clone(), path.clone(), w_rx)) => e,
        //Err(e) = spawn(agent_thread(state, callback, path)) => e,
        else => Error::exited("Main Thread")
    );

    let mut threads = THREAD_CHANNELS.lock().await;
    threads.0 = None;

    Err(err)
}

#[async_backtrace::framed]
pub async fn call_thread(thread: Threads) -> Result<String, Error> {
    let (o_tx, o_rx) = oneshot::channel::<ChannelType>();
    let threads = THREAD_CHANNELS.lock().await;
    match thread {
        Threads::Wallet(method) => threads.0.as_ref().ok_or(Error::exited("Wallet Channel"))?.send((o_tx, method)).await?,
    }
    o_rx.await?
}
//PRIVATE

async fn spawn<T>(task: T) -> Result<(), Error>
    where
        T: std::future::Future<Output = Result<(), Error>> + Send + 'static
{
    tokio::spawn(async_backtrace::location!().frame(task)).await.map_err(|e| Error::tokio_join(e))?
}

#[async_backtrace::framed]
async fn internet_thread(state: State) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(1000));
    loop {
        let connected = match Request::process_result(reqwest::get("https://google.com").await) {
            Ok(_) => true,
            Err(Error::NoInternet{backtrace: _}) => false,
            Err(e) => {
                log::info!("Connection Error: {:?}", e);
                return Err(e);
            },
        };
        state.set(Field::Internet(Some(connected))).await?;
        interval.tick().await;
    }
}

#[async_backtrace::framed]
async fn price_thread(state: State) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(10_000));
    loop {
        state.set(Field::Price(Some(PriceGetter::get(None).await?))).await?;
        interval.tick().await;
    }
}

#[async_backtrace::framed]
async fn wallet_thread(
    state: State,
    callback: Callback,
    path: PathBuf,
    w_rx: WalletReceiver
) -> Result<(), Error> {
    let wallet = Wallet::new(callback, path.clone()).await?;
    Err(tokio::select!(
        Err(e) = spawn(wallet_method_thread(wallet.clone(), w_rx)) => e,
        Err(e) = spawn(wallet_sync_thread(wallet.clone())) => e,
        Err(e) = spawn(wallet_refresh_thread(wallet, state)) => e,
        else => Error::exited("Wallet Thread")
    ))
}

#[async_backtrace::framed]
async fn wallet_sync_thread(wallet: Wallet) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(10_000));
    loop {
        Request::filter_error(wallet.sync().await)?;
        interval.tick().await;
    }
}

#[async_backtrace::framed]
async fn wallet_refresh_thread(wallet: Wallet, state: State) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(10_000));
    loop {
        wallet.refresh_state(&state).await?;
        interval.tick().await;
    }
}

#[async_backtrace::framed]
async fn wallet_method_thread(wallet: Wallet, mut recv: WalletReceiver) -> Result<(), Error> {
    loop {
        let (o_tx, method) = recv.recv().await.ok_or(Error::exited("Wallet Channel"))?;
        o_tx.send(async { match method {
            WalletMethod::GetNewAddress => Ok(wallet.get_new_address().await?),
            WalletMethod::GetFees(amount) => {
                Ok(serde_json::to_string(&wallet.get_fees(amount).await?)?)
            },
            WalletMethod::BuildTransaction(address, amount, fee, price) => {
                Ok(serde_json::to_string(&wallet.build_transaction(&address, amount, fee, price).await?)?)
            },
            WalletMethod::BroadcastTransaction(tx) => {
                wallet.broadcast_transaction(&tx).await?;
                Ok(String::new())
            },
        }}.await).map_err(|e| Error::exited(format!("{:?}", e).as_str()))?;
    }
}

#[async_backtrace::framed]
async fn agent_thread(state: State, callback: Callback, path: PathBuf) -> Result<(), Error> {
    let agent = MessagingAgent::new(callback, path).await?;
    Err(tokio::select!(
        Err(e) = spawn(agent_sync_thread(agent.clone())) => e,
        Err(e) = spawn(agent_refresh_thread(agent, state)) => e,
        else => Error::exited("Agent Thread")
    ))
}

#[async_backtrace::framed]
async fn agent_sync_thread(agent: MessagingAgent) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(10_000));
    loop {
        agent.sync().await?;
        interval.tick().await;
    }
}

#[async_backtrace::framed]
async fn agent_refresh_thread(agent: MessagingAgent, state: State) -> Result<(), Error> {
    let mut interval = time::interval(Duration::from_millis(5_000));
    loop {
        agent.refresh_state(&state).await?;
        interval.tick().await;
    }
}

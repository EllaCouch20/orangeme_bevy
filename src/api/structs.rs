use super::Error;

use super::pub_structs::DartMethod;

use std::sync::Arc;

use chrono::{Utc, DateTime as ChronoDateTime};
use flutter_rust_bridge::DartFnFuture;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use reqwest::Response;
use std::time::Duration;
use tokio::time;

pub type Callback = Arc<Mutex<dyn Fn(DartMethod) -> DartFnFuture<Option<String>> + 'static + Sync + Send>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct DateTime {
    pub date: ChronoDateTime<Utc>
}

impl DateTime {
    pub fn from_timestamp(timestamp: u64) -> Result<DateTime, Error> {
        Ok(DateTime{date:
            ChronoDateTime::<Utc>::from_timestamp(timestamp as i64, 0).ok_or(Error::bad_request("DateTime::from_timestamp", "Could not create date from timestamp"))?
        })
    }
    pub fn now() -> DateTime {
        DateTime{date: Utc::now()}
    }
    pub fn timestamp(&self) -> u64 {
        Some(self.date.timestamp()).filter(|t| *t >= 0).expect("timestamp was negative") as u64
    }
    pub fn format(&self, fmt: &str) -> String {
        self.date.format(fmt).to_string()
    }
}

pub struct Request {}

impl Request {
    #[async_backtrace::framed]
    pub async fn get(url: &str) -> Result<Response, Error> {
        Ok(Self::repeat(|| Box::pin(reqwest::get(url.to_string()))).await?)
    }

    pub fn check_error<E: std::fmt::Debug>(err: &E) -> bool {
        let e = format!("{:?}", err);
        e.contains("failed to lookup address information: No address associated with hostname") ||
        e.contains("Network is unreachable") ||
        e.contains("Software caused connection abort")
    }

    pub fn process_result<T, E>(res: Result<T, E>) -> Result<T, Error>
        where E: std::fmt::Debug + Into<Error>
    {
        res.map_err(|err|
            if Self::check_error(&err) {Error::no_internet()} else {err.into()}
        )
    }

    pub fn filter_error(res: Result<(), Error>) -> Result<(), Error> {
        if matches!(res, Err(Error::NoInternet{backtrace: _})) {Ok(())} else {res}
    }

    #[async_backtrace::framed]
    pub async fn repeat<T, O, E>(task: T) -> Result<O, E>
        where
            E: std::fmt::Debug,
            T: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<O, E>> + Send + 'static>>
    {
        let mut interval = time::interval(Duration::from_millis(1000));
        loop {
            match task().await {
                Ok(res) => {return Ok(res);},
                Err(e) if Self::check_error(&e) => {interval.tick().await;},
                Err(e) => {return Err(e);},
            }
        }
    }
}

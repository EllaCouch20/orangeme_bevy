use super::Error;

use super::pub_structs::{Platform, PageName, DartMethod};
use super::state::{StateManager, State, Field};
use super::threads::start_threads;

use std::{thread, time};
use std::path::PathBuf;
use std::sync::Arc;

use flutter_rust_bridge::DartFnFuture;
use simple_database::SqliteStore;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[async_backtrace::framed]
pub async fn rustStart (
    path: String,
    platform: Platform,
    callback: impl Fn(DartMethod) -> DartFnFuture<Option<String>> + 'static + Sync + Send,
) -> Result<(), String> {

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    
    //#[cfg(any(target_os = "ios", target_os = "macos"))]
    //oslog::OsLogger::new("frb_user").level_filter(log::LevelFilter::Info).init();
    thread::sleep(time::Duration::from_millis(500));//TODO: loggers need time to initialize maybe find an async solution

    let callback = Arc::new(Mutex::new(callback));

    let path = PathBuf::from(&path);
    let state = State::new::<SqliteStore>(path.clone()).await?;
    state.set(Field::Path(Some(path.clone()))).await?;
    state.set(Field::Platform(Some(platform))).await?;

    Ok(start_threads(state, callback, path).await?)
}

#[allow(non_snake_case)]
#[async_backtrace::framed]
pub async fn getPage(path: String, page: PageName) -> Result<String, String> {
    Ok(StateManager::new(State::new::<SqliteStore>(PathBuf::from(&path)).await?).get(page).await?)
}

#[allow(non_snake_case)]
pub fn clearData(path: String) -> Result<(), String> {
    std::fs::remove_dir_all(PathBuf::from(path)).unwrap();
    Ok(())
}
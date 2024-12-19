use flutter_rust_bridge::handler::{SimpleHandler, SimpleExecutor, NoOpErrorListener};
use flutter_rust_bridge::{BaseAsyncRuntime, SimpleThreadPool, JoinHandle};
use std::future::Future;

pub struct MyCustomAsyncRuntime {
    runtime: tokio::runtime::Runtime
}

impl BaseAsyncRuntime for MyCustomAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }
}

impl Default for MyCustomAsyncRuntime {
    fn default() -> Self {
        MyCustomAsyncRuntime{
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(8)
                .enable_io()
                .enable_time()
                .build().unwrap()
        }
    }
}

pub type MyUnmodifiedHandler = SimpleHandler<
    SimpleExecutor<NoOpErrorListener, SimpleThreadPool, MyCustomAsyncRuntime>,
    NoOpErrorListener,
>;

lazy_static::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: MyUnmodifiedHandler = MyUnmodifiedHandler::new(
        SimpleExecutor::<NoOpErrorListener, SimpleThreadPool, MyCustomAsyncRuntime>::new(
            NoOpErrorListener, Default::default(), Default::default()
        ),
        NoOpErrorListener
    );
}

use std::future::Future;
use futures::future::BoxFuture;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::Error;
use tokio::task::JoinHandle;

use common::logging;

pub trait TcpRecvCallback<S>: Send + Sync {
    fn call(&self, stream: TcpStream, state: S) -> BoxFuture<'static, ()>;
}

impl<T, F, S> TcpRecvCallback<S> for T
where
    T: Fn(TcpStream, S) -> F + Send + Sync,
    F: Future<Output = ()> + 'static + Send + Sync,
    S: Send + Sync,
{
    fn call(&self, stream: TcpStream, state: S) -> BoxFuture<'static, ()> {
        Box::pin(self(stream, state))
    }
}

pub async fn listen<S: Send + Sync + Clone + 'static>(
    addr: &str,
    state: S,
    callback: impl TcpRecvCallback<S> + 'static,
) -> Result<JoinHandle<()>, Error> {
    let listener = TcpListener::bind(addr).await?;

    Ok(tokio::spawn(recv_loop(listener, state, callback)))
}

async fn recv_loop<S: Send + Sync + Clone>(
    listener: TcpListener,
    state: S,
    callback: impl TcpRecvCallback<S>,
) {
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                logging::info!("New connection from: {}", addr);
                tokio::spawn(callback.call(socket, state.clone()));
            }
            Err(e) => logging::error!("Failed to accept connection: {}", e)
        }
    }
}

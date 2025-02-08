use std::future::Future;
use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::tcp_server::connection::Connection;

pub trait PacketHandlerCallback<S>: Send + Sync {
    fn call(&self, state: S, item: PacketHandlerItem) -> BoxFuture<'static, ()>;
}

impl<T, F, S> PacketHandlerCallback<S> for T
where
    T: Fn(S, PacketHandlerItem) -> F + Send + Sync,
    F: Future<Output = ()> + 'static + Send + Sync,
    S: Send + Sync,
{
    fn call(&self, state: S, item: PacketHandlerItem) -> BoxFuture<'static, ()> {
        Box::pin(self(state, item))
    }
}

#[derive(Clone)]
pub enum PacketHandlerItem {
    NewConnection(Connection),
    DropConnection(u32),
    Packet(u32, Box<Vec<u8>>),
}

#[derive(Clone)]
pub struct PacketHandler(pub mpsc::UnboundedSender<PacketHandlerItem>);

impl PacketHandler {
    pub fn new<S: Send + Sync + Clone + 'static>(
        state: S,
        callback: impl PacketHandlerCallback<S> + 'static,
    ) -> Self {
        let (tx, rx) =
            mpsc::unbounded_channel();
        tokio::spawn(async move {
            packet_handler_loop(rx, state, callback).await;
        });

        Self(tx)
    }
}

async fn packet_handler_loop<S: Send + Sync + Clone + 'static>(
    mut rx: mpsc::UnboundedReceiver<PacketHandlerItem>,
    state: S,
    callback: impl PacketHandlerCallback<S>,
) {
    while let Some(item) = rx.recv().await {
        callback.call(state.clone(), item).await;
    }
}

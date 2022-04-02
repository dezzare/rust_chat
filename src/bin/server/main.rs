use async_std::net;
use async_std::prelude::*;
use async_std::task;
use rust_chat::utils::ChatResult;
use std::sync::Arc;

mod chats;
mod chats_map;
mod connection;

use connection::handle;

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("server ADDRESS");
    let chat_table = Arc::new(chats_map::ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(addr).await?;
        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let chats = chat_table.clone();
            task::spawn(async {
                log_error(handle(socket, chats).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        println!("Erro: {}", error);
    }
}

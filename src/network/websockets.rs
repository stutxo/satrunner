use bevy::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::WebSocketError;
use gloo_net::websocket::{futures::WebSocket, Message};

use gloo_timers::future::TimeoutFuture;

use speedy::Writable;
use wasm_bindgen_futures::spawn_local;

use crate::game_util::resources::NetworkStuff;

use super::messages::PlayerInput;

pub const DELAY: u32 = 200;

pub fn websocket(mut server: ResMut<NetworkStuff>) {
    let ws = WebSocket::open("ws://localhost:3030/run").unwrap();
    let (mut write, mut read) = ws.split();

    let (send_tx, mut send_rx) = futures::channel::mpsc::channel::<PlayerInput>(1000);
    let (mut read_tx, read_rx) = futures::channel::mpsc::channel::<Vec<u8>>(20000);

    server.write = Some(send_tx);
    server.read = Some(read_rx);

    spawn_local(async move {
        while let Some(message) = send_rx.next().await {
            let message = message.write_to_vec().unwrap();

            //////
            /// //
            /// //
            /// //
            /// /
            /// //
            //TimeoutFuture::new(DELAY).await;
            //info!("sending message, {:?}", message);
            write.send(Message::Bytes(message)).await.unwrap();
        }
    });

    spawn_local(async move {
        while let Some(result) = read.next().await {
            ////
            /// /
            /// /
            /// /
            ///
            ///
            //TimeoutFuture::new(DELAY).await;
            //info!("Got message {:?}", result);
            match result {
                Ok(Message::Bytes(msg)) => match read_tx.try_send(msg) {
                    Ok(()) => {}
                    Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                },

                Ok(Message::Text(_)) => {}

                Err(e) => match e {
                    WebSocketError::ConnectionError => {}
                    WebSocketError::ConnectionClose(_) => {}
                    WebSocketError::MessageSendError(_) => {}
                    _ => {}
                },
            }
        }
    });
}

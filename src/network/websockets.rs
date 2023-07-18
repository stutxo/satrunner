use bevy::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::WebSocketError;
use gloo_net::websocket::{futures::WebSocket, Message};

use speedy::Writable;
use wasm_bindgen_futures::spawn_local;

use crate::game_util::resources::NetworkStuff;
use crate::GameStage;

use super::messages::ClientMessage;

//pub const DELAY: u32 = 200;
//use gloo_timers::future::TimeoutFuture;

pub fn websocket(
    mut network_stuff: ResMut<NetworkStuff>,
    mut next_state: ResMut<NextState<GameStage>>,
) {
    let ws = WebSocket::open("ws://localhost:3030/run").unwrap();
    let (mut write, mut read) = ws.split();

    let (send_tx, mut send_rx) = futures::channel::mpsc::channel::<ClientMessage>(1000);
    let (mut read_tx, read_rx) = futures::channel::mpsc::channel::<Vec<u8>>(20000);

    network_stuff.write = Some(send_tx);
    network_stuff.read = Some(read_rx);

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
                    WebSocketError::ConnectionError => {
                        error!("connection error: {:?}", e);
                    }
                    WebSocketError::ConnectionClose(_) => {
                        error!("connection closed error: {:?}", e);
                    }
                    WebSocketError::MessageSendError(_) => {
                        error!("msg send error: {:?}", e);
                    }
                    _ => {}
                },
            }
        }
    });

    next_state.set(GameStage::Menu);
}

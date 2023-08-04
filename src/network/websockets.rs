use bevy::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::WebSocketError;
use gloo_net::websocket::{futures::WebSocket, Message};

use speedy::Writable;
use wasm_bindgen_futures::spawn_local;

use crate::game_util::resources::{NetworkStuff, PingTimer};

use super::messages::ClientMessage;

// pub const DELAY: u32 = 200;
// use gloo_timers::future::TimeoutFuture;

pub fn websocket(mut network_stuff: ResMut<NetworkStuff>, mut ping: ResMut<PingTimer>) {
    //let ws = WebSocket::open("ws://0.0.0.0:3030/run").unwrap();
    let ws = WebSocket::open("wss://satrunner.gg/run").unwrap();
    let (mut write, mut read) = ws.split();

    let (send_tx, mut send_rx) = futures::channel::mpsc::channel::<ClientMessage>(1000);
    let (mut read_tx, read_rx) = futures::channel::mpsc::channel::<Vec<u8>>(20000);

    let (cancel_tx, cancel_rx) = futures::channel::mpsc::channel::<()>(1);
    let mut cancel_tx_clone = cancel_tx.clone();

    network_stuff.write = Some(send_tx);
    network_stuff.read = Some(read_rx);
    ping.disconnected_rx = Some(cancel_rx);
    ping.disconnected_tx = Some(cancel_tx);

    spawn_local(async move {
        while let Some(message) = send_rx.next().await {
            let message = message.write_to_vec().unwrap();

            // TimeoutFuture::new(DELAY).await;
            //info!("sending message, {:?}", message);
            let send = write.send(Message::Bytes(message)).await;

            match send {
                Ok(_) => {}
                Err(e) => {
                    info!("{:?}", e)
                }
            }
        }
    });

    spawn_local(async move {
        while let Some(result) = read.next().await {
            // TimeoutFuture::new(DELAY).await;
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
                        cancel_tx_clone.send(()).await.unwrap();
                        break;
                    }
                    WebSocketError::ConnectionClose(_) => {
                        error!("connection closed error: {:?}", e);
                        cancel_tx_clone.send(()).await.unwrap();
                        break;
                    }
                    WebSocketError::MessageSendError(_) => {
                        error!("msg send error: {:?}", e);
                    }
                    _ => {}
                },
            }
        }
    });
}

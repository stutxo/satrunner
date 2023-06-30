use bevy::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::WebSocketError;
use gloo_net::websocket::{futures::WebSocket, Message};

use wasm_bindgen_futures::spawn_local;

use crate::{
    game_util::resources::Server,
    network::messages::{ClientMsg, GameState},
};

pub fn websocket(mut server: ResMut<Server>) {
    let ws = WebSocket::open("ws://localhost:3030/play").unwrap();
    let (mut write, mut read) = ws.split();

    let (send_tx, mut send_rx) = futures::channel::mpsc::channel::<ClientMsg>(1000);
    let (mut read_tx, read_rx) = futures::channel::mpsc::channel::<GameState>(20000);

    server.write = Some(send_tx);
    server.read = Some(read_rx);

    spawn_local(async move {
        while let Some(message) = send_rx.next().await {
            match serde_json::to_string::<ClientMsg>(&message) {
                Ok(new_input) => {
                    //info!("Sending message: {:?}", new_input);
                    write.send(Message::Text(new_input)).await.unwrap();
                }
                Err(e) => {
                    info!("Failed to parse message as Vec2: {:?}", e);
                }
            }
        }
    });

    spawn_local(async move {
        while let Some(result) = read.next().await {
            match result {
                Ok(Message::Text(msg)) => match serde_json::from_str::<GameState>(&msg) {
                    Ok(new_player_vec) => match read_tx.try_send(new_player_vec) {
                        Ok(()) => {}
                        Err(e) => info!("Error sending message: {} CHANNEL FULL???", e),
                    },
                    Err(e) => {
                        info!("Failed to parse message: {:?}", e);
                    }
                },
                Ok(Message::Bytes(_)) => {}

                Err(e) => match e {
                    WebSocketError::ConnectionError => {}
                    WebSocketError::ConnectionClose(_) => {
                        //
                    }
                    WebSocketError::MessageSendError(_) => {}
                    _ => {}
                },
            }
        }
    });
}

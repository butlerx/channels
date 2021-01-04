use crate::routes::Responce;
use futures::{SinkExt, StreamExt};
use nats::asynk::Connection;
use prometheus::IntGaugeVec;
use serde::{Deserialize, Serialize};
use warp::{
    ws::{Message, WebSocket, Ws},
    Reply,
};

lazy_static! {
    static ref CONNECTED_CLIENTS: IntGaugeVec = register_int_gauge_vec!(
        opts!("connected_clients", "Connected Clients"),
        &["group", "subject"]
    )
    .expect("metric can be created");
}

#[derive(Deserialize, Serialize)]
pub struct Args {
    group: String,
}

pub async fn handler(subject: String, query: Args, ws: Ws, nc: Connection) -> Responce<impl Reply> {
    Ok(ws.on_upgrade(move |socket| handle_connection(socket, nc, subject, query.group)))
}

async fn handle_connection(ws: WebSocket, nc: Connection, subject: String, group: String) {
    let (mut sender, mut rcv) = ws.split();

    info!("Connection Opened; subject={}, group={}", subject, group);
    CONNECTED_CLIENTS
        .with_label_values(&[&group, &subject])
        .inc();

    let (ws_subject, ws_group, mut sub) = (
        subject.clone(),
        group.clone(),
        nc.queue_subscribe(&subject, &group).await.unwrap(),
    );
    tokio::task::spawn(async move {
        while let Some(msg) = sub.next().await {
            let txt = String::from_utf8(msg.data).unwrap_or("Invalid UTF-8".to_string());
            debug!(
                "Message recieved from nats; msg={} subject={} group={}",
                txt, ws_subject, ws_group
            );
            let _ = sender.send(Message::text(txt)).await;
        }
    });
    while let Some(event) = rcv.next().await {
        match event {
            Ok(msg) => {
                debug!(
                    "Message recieved from websocket; msg={:?} subject={} group={}",
                    msg, subject, group
                );
                if let Ok(txt) = msg.to_str() {
                    if let Err(err) = nc.publish(&subject, txt).await {
                        error!(
                            "Error publishing message; error={} subject={} group={}",
                            err, subject, group
                        );
                    }
                }
            }
            Err(err) => {
                error!(
                    "websocket error; error={} subject={} group={}",
                    err, subject, group
                );
                break;
            }
        }
    }
    info!("Connection Dropped; subject={}, group={}", subject, group);
    CONNECTED_CLIENTS
        .with_label_values(&[&group, &subject])
        .dec();
}

use crate::Responce;
use futures::{SinkExt, StreamExt};
use nats::asynk::Connection;
use warp::{Reply, ws::{Message, WebSocket, Ws}};

pub async fn handler(subject: String, ws: Ws, nc:Connection) -> Responce<impl Reply>  {
    Ok(ws.on_upgrade(|socket| handle_connection(socket, nc, subject)))
}

async fn handle_connection(ws: WebSocket, nc:Connection, subject: String) {
    info!("Connection Opened");
    let (mut sender, mut rcv) = ws.split();
    let (ws_subject, mut sub) = (subject.clone(), nc.subscribe(&subject).await.unwrap());
    tokio::task::spawn(async move {
        while let Some(msg) = sub.next().await {
            let txt = String::from_utf8(msg.data).unwrap_or("Invalid UTF-8".to_string());
            debug!("Message recieved from nats; msg={}, subject={}", txt, ws_subject);
            let _ = sender.send(Message::text(txt)).await;
        }
    });
    while let Some(event) = rcv.next().await {
        match event {
            Ok(msg) => {
                debug!("Message recieved from websocket; msg={:?}, subject={}", msg, subject);
                if let Ok(txt) = msg.to_str() {
                    if let Err(err) = nc.publish(&subject, txt).await  {
                        error!("Error publishing message; error={} subject={}", err, subject);
                    }
                }
            }
            Err(err) => {
                error!("websocket error: {}", err);
                break;
            }
        }
    };
    info!("Connection Dropped");
}

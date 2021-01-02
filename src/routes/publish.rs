use crate::Responce;
use nats::asynk::Connection;
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    msg: String,
}

pub async fn handler(subject: String, body: Event, nc: Connection) -> Responce<impl Reply> {
    debug!("Event Recieved; subject={} body={:?}", subject, body);
    if let Ok(txt) = serde_json::to_string(&body) {
        if let Err(err) = nc.publish(&subject, &txt).await {
            error!("Error publishing message: {}", err);
        }
    }
    Ok(StatusCode::OK)
}

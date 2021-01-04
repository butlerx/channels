use crate::routes::Responce;
use nats::asynk::Connection;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use warp::{reply, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    msg: String,
    processed: bool,
}

pub async fn handler(subject: String, body: Value, nc: Connection) -> Responce<impl Reply> {
    debug!("Event Recieved; subject={} body={:?}", subject, body);
    let txt = serde_json::to_string(&body).unwrap();
    if let Err(err) = nc.publish(&subject, &txt).await {
        error!("Error publishing message: {}", err);
        Err(warp::reject())
    } else {
        Ok(reply::json(&Event {
            msg: "Message published".to_string(),
            processed: true,
        }))
    }
}

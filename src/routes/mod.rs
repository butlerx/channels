use crate::config::Nats;
use nats::asynk::Connection;
use warp::{Filter, Rejection};

pub mod health;
pub mod metrics;
pub mod publish;
pub mod subscribe;

type Responce<T> = std::result::Result<T, Rejection>;

pub async fn with_nats(
    nats: &Nats,
) -> impl Filter<Extract = (Connection,), Error = std::convert::Infallible> + Clone {
    let opts = if let Some(creds_path) = &nats.creds {
        nats::Options::with_credentials(creds_path)
    } else {
        nats::Options::new()
    };
    let nc = opts
        .with_name("Channels Server")
        .tls_required(nats.tls)
        .connect_async(&nats.url)
        .await
        .expect("Unable to connect to Nats server");
    warp::any().map(move || nc.clone())
}

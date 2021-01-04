use crate::config::Nats;
use nats::asynk::Connection;
use prometheus::{HistogramVec, IntCounter, IntCounterVec};
use warp::filters::log::Info;
use warp::Filter;

lazy_static! {
    static ref INCOMING_REQUESTS: IntCounter =
        register_int_counter!("incoming_requests", "Incoming Requests")
            .expect("metric can be created");
    static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = register_int_counter_vec!(
        opts!("response_code", "Response Codes"),
        &["method", "path", "status", "type"]
    )
    .expect("metric can be created");
    static ref RESPONSE_TIME_COLLECTOR: HistogramVec = register_histogram_vec!(
        histogram_opts!("response_time", "Response Times"),
        &["method", "path"]
    )
    .expect("metric can be created");
}

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

pub fn metrics(info: Info) {
    let (method, path, status_code) = (
        info.method().to_string(),
        info.path(),
        info.status().as_u16(),
    );
    INCOMING_REQUESTS.inc();
    RESPONSE_TIME_COLLECTOR
        .with_label_values(&[&method, &path])
        .observe(info.elapsed().as_secs_f64());
    match status_code {
        500..=599 => RESPONSE_CODE_COLLECTOR
            .with_label_values(&[&method, &path, &status_code.to_string(), "500"])
            .inc(),
        400..=499 => RESPONSE_CODE_COLLECTOR
            .with_label_values(&[&method, &path, &status_code.to_string(), "400"])
            .inc(),
        300..=399 => RESPONSE_CODE_COLLECTOR
            .with_label_values(&[&method, &path, &status_code.to_string(), "300"])
            .inc(),
        200..=299 => RESPONSE_CODE_COLLECTOR
            .with_label_values(&[&method, &path, &status_code.to_string(), "200"])
            .inc(),
        100..=199 => RESPONSE_CODE_COLLECTOR
            .with_label_values(&[&method, &path, &status_code.to_string(), "100"])
            .inc(),
        _ => (),
    };
}

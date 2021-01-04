#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate stderrlog;

mod config;
mod routes;

use config::Config;
use futures::join;
use prometheus::{HistogramVec, IntCounter, IntCounterVec, IntGaugeVec};
use std::{io::Result, net::SocketAddr, path::PathBuf};
use structopt::StructOpt;
use warp::Filter;

lazy_static! {
    static ref INCOMING_REQUESTS: IntCounter =
        register_int_counter!("incoming_requests", "Incoming Requests")
            .expect("metric can be created");
    static ref CONNECTED_CLIENTS: IntGaugeVec = register_int_gauge_vec!(
        opts!("connected_clients", "Connected Clients"),
        &["group", "subject"]
    )
    .expect("metric can be created");
    static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = register_int_counter_vec!(
        opts!("response_code", "Response Codes"),
        &["env", "statuscode", "type"]
    )
    .expect("metric can be created");
    static ref RESPONSE_TIME_COLLECTOR: HistogramVec =
        register_histogram_vec!(histogram_opts!("response_time", "Response Times"), &["env"])
            .expect("metric can be created");
}

/// NATS Websocket server
#[derive(Debug, StructOpt)]
struct Args {
    /// Configuration file config path
    #[structopt(short, long, parse(from_os_str), default_value = "configs/config.toml")]
    config: PathBuf,
    /// Silence all output
    #[structopt(short, long)]
    quiet: bool,
    /// Increase message verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Print default config
    #[structopt(short, long)]
    print: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    if args.print {
        return Ok(Config::print_default());
    }

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let conf = Config::from_file(&args.config)?;
    info!("config loaded; path={:?}", args.config);

    let metrics_route = warp::path!("metrics").and_then(routes::metrics::handler);
    let health = warp::path!("health").and_then(routes::health::handler);
    let events = warp::path!("api" / "v1" / "events" / String / ..)
        .and(warp::query::<routes::subscribe::Args>())
        .and(warp::ws())
        .and(routes::with_nats(&conf.nats).await)
        .and_then(routes::subscribe::handler);
    let publish = warp::path!("api" / "v1" / "events" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(routes::with_nats(&conf.nats).await)
        .and_then(routes::publish::handler);

    let private_routes = warp::fs::dir("client/build")
        .or(metrics_route)
        .or(health)
        .or(events)
        .or(publish.clone());

    let public_routes = health
        .or(publish.clone())
        .or(warp::any().and(warp::fs::file("README.md")));

    info!(
        "Server started; type=private, address={}",
        conf.server.private
    );
    let private_server =
        warp::serve(private_routes).run(conf.server.private.parse::<SocketAddr>().unwrap());

    info!(
        "Server started; type=public, address={}",
        conf.server.public
    );
    let public_server =
        warp::serve(public_routes).run(conf.server.public.parse::<SocketAddr>().unwrap());

    join!(private_server, public_server);
    info!("Server shutting down");
    Ok(())
}

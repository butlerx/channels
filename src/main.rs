#[macro_use]
extern crate log;
extern crate stderrlog;

mod config;
mod routes;

use config::{Nats, Config};
use nats::{self, asynk::Connection};
use std::{net::SocketAddr,io::Result, path::PathBuf};
use structopt::StructOpt;
use warp::{Rejection, Filter};

type Responce<T> = std::result::Result<T, Rejection>;

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

    debug!("loading config; path={:?}", args.config);
    let conf = Config::from_file(&args.config)?;
    info!("config loaded; path={:?}", args.config);

    let health = warp::path!("health").and_then(routes::health::handler);
    let events = warp::path!("api" / "v1"/ "events" / String/ ..)
        .and(warp::ws())
        .and(with_nats(&conf.nats).await)
        .and_then(routes::subscribe::handler);
    let publish = warp::path!("api" / "v1"/ "events" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_nats(&conf.nats).await)
        .and_then(routes::publish::handler);

    let routes = warp::fs::dir("static").or(health)
        .or(events)
        .or(publish);

    warp::serve(routes).run(conf.listen.parse::<SocketAddr>().unwrap()).await;
    Ok(())
}

async fn with_nats(nats: &Nats) -> impl Filter<Extract = (Connection,), Error = std::convert::Infallible> + Clone {
    let opts = if let Some(creds_path) = &nats.creds {
        nats::Options::with_credentials(creds_path)
    } else {
        nats::Options::new()
    };
    let nc = opts
        .with_name("Channels Server")
        .tls_required(nats.tls)
        .connect_async(&nats.url).await.unwrap();
    warp::any().map(move || nc.clone())
}

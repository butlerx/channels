#[macro_use]
extern crate log;
extern crate stderrlog;

mod config;
mod pages;

use config::{Nats, Config};
use futures::{SinkExt, StreamExt};
use std::{io::Result, path::PathBuf};
use structopt::StructOpt;
use warp::{Filter, ws::{Message, WebSocket}};
use nats::{self, asynk::Connection};

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

    let event = warp::path!("api" / "v1"/ "events" / String/ ..)
        .and(warp::ws()).and(with_nats(conf.nats).await)
        .map(|subject: String,  ws: warp::ws::Ws, nc: Connection| {
            ws.on_upgrade(|socket| event_handler(socket, nc, subject))
        });

    warp::serve(warp::path::end().map(|| warp::reply::html(pages::INDEX_HTML)).or(event)).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

async fn with_nats(nats: Nats) -> impl Filter<Extract = (Connection,), Error = std::convert::Infallible> + Clone {
    let opts = if let Some(creds_path) = nats.creds {
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

async fn event_handler(ws: WebSocket, nc:Connection, subject: String) {
    info!("Connection Opened");
    let (mut sender, mut rcv) = ws.split();
    let mut sub = nc.subscribe(&subject).await.unwrap();
    tokio::task::spawn(async move {
        while let Some(msg) = sub.next().await {
            let txt = String::from_utf8(msg.data).unwrap_or("Invalid UTF-8".to_string());
            debug!("Message recieved from nats: {}", txt);
            let t = sender.send(Message::text(txt)).await;
            info!("{:?}", t)
        }
    });
    while let Some(event) = rcv.next().await {
        match event {
            Ok(msg) => {
                debug!("Message recieved from websocket: {:?}", msg);
                if let Ok(txt) = msg.to_str() {
                    if let Err(err) = nc.publish(&subject, txt).await  {
                        error!("Error publishing message: {}", err);
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

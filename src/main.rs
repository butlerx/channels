use nats;
use std::{io::Result, net::TcpListener, thread::spawn};
use structopt::StructOpt;
use tungstenite::{server::accept, Message};

/// NATS Websocket server
#[derive(Debug, StructOpt)]
struct Args {
    /// The nats server URLs (separated by comma) (default "nats://127.0.0.1:4222")
    #[structopt(long, short, default_value = "nats://127.0.0.1:4222")]
    url: String,

    /// The Websocket Listen Address (default "127.0.0.1:9001")
    #[structopt(long, short, default_value = "127.0.0.1:9001")]
    listen: String,

    /// User Credentials File
    #[structopt(long = "creds")]
    creds: Option<String>,

    /// Use TLS Secure Connection
    #[structopt(short = "tls")]
    tls: bool,

    /// The subject to default to
    #[structopt(default_value = "channels.all")]
    subject: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let opts = if let Some(creds_path) = args.creds {
        nats::Options::with_credentials(creds_path)
    } else {
        nats::Options::new()
    };

    let server = TcpListener::bind(args.listen)?;
    let nc = opts
        .with_name("Channels Server")
        .tls_required(args.tls)
        .connect(&args.url)?;
    for stream in server.incoming() {
        let (publisher, subject) = (nc.clone(), args.subject.clone());
        let sub = nc.subscribe(&subject).unwrap();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                if let Some(msg) = sub.next() {
                    websocket
                        .write_message(Message::Text(msg.to_string()))
                        .unwrap();
                }
                match websocket.read_message() {
                    Ok(Message::Text(msg)) => {
                        publisher.publish(&subject, msg).unwrap();
                    }
                    Ok(Message::Binary(msg)) => {
                        websocket.write_message(Message::Binary(msg)).unwrap()
                    }
                    Ok(Message::Close(_)) => {
                        sub.close().unwrap();
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
    Ok(())
}

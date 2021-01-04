# channels

Channels is a distributed multi-channel websocket based streaming data service.
Channels Server allows for publishing arbitrary data that can be consumed by
multiple consumers.

Channels are low cost to create and delete allowing for user to send generate
shared secrets to send and consume data from without needing to configure
servers.

Channels is designed for easy webhook integration, etl and buffering of
distributed workloads.

## Spark Gapping

Channels has to endpoints, a public endpoint that only excepts post requests and
a private with all endpoint available. This allow channels to accept webhooks
from multiple servers without having to have the consumption endpoint available
publicly.

## Running

**requires: NATS**

`cargo run`

### Configuration

## Development

To start the server in dev mode first run from the root of the repo

```bash
cargo watch -x 'run -- -vvv' --ignore client
```

This Start the public API Server on port `9002` and the private server on
`9001`.

Then from the `client` directory run

```bash
yarn start
```

This will start a dev server for the client app for the public endpoint on port
`3000` that will proxy all requests to the public api server

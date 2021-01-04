#!/usr/bin/env node

// Requires `npm i websocket fetch` to work

const WebSocket = require('websocket').w3cwebsocket;
const fetch = require('node-fetch');

const listen = 'unstructured';
const group = 'Node Etl example';

const ws = new WebSocket(`ws://localhost:3030/api/v1/events/${listen}?group=${group}`);

ws.addEventListener('message', (event) => {
  fetch('http://localhost:3030/api/v1/events/structured', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ msg: event.data, etl: true, timestamp: Date.now() }),
  })
    .then((resp) => resp.json())
    .then((data) => console.log(data))
    .catch((err) => console.error(err));
});

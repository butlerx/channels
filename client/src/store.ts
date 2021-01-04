import { writable, get } from 'svelte/store';

export const messages = writable([]);
export const channels = writable([]);

const sockets: { [string]: Websocket } = {};

channels.subscribe(c => {
  const subscribed = Object.keys(sockets);
  c.filter(channel => !subscribed.includes(channel)).forEach(subscribe);
});

export function subscribe(channel: string) {
  const ws = new WebSocket(
    `ws://${location.host}/api/v1/events/${channel}?group=${guidGenerator()}`,
  );
  ws.addEventListener('message', event => {
    messages.set([
      ...get(messages),
      { channel, text: parseMsg(event.data), timestamp: new Date().toLocaleString() },
    ]);
  });
  sockets[channel] = ws;
  channels.set([...get(channels), channel]);
}

export function send(channel: string, message: string) {
  if (channel in sockets && sockets[channel].readyState <= 1) {
    sockets[channel].send(message);
  }
}

function parseMsg(msg: string) {
  try {
    return JSON.parse(msg);
  } catch {
    return msg;
  }
}

function guidGenerator() {
  const S4 = () => (((1 + Math.random()) * 0x10000) | 0).toString(16).substring(1);
  return `js-client-${S4()}${S4()}-${S4()}-${S4()}-${S4()}-${S4()}${S4()}${S4()}`;
}

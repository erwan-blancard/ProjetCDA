import ReconnectingWebSocket from 'reconnecting-websocket';

export let socket;
export let clientId;

export function initSocket(onMessage) {
  socket = new ReconnectingWebSocket('ws://' + window.location.hostname + ':8080/ws/');
  socket.addEventListener('open', () => console.log('WebSocket connecté'));
  socket.addEventListener('message', evt => {
    const msg = JSON.parse(evt.data);
    onMessage(msg);
  });
}

export function send(type, data = {}) {
  socket.send(JSON.stringify({ type, data }));
}

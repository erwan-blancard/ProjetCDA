import * as THREE        from 'three';
import { initGame, setHand, onOpponentPlay, onOpponentDraw } from './game.js';
import { initSocket, send } from './network.js';

let players = [];
let yourTurn = false;

function handleServerMessage(msg) {
  switch (msg.type) {
    case 'RoomCreated':
      alert('Room créée: ' + msg.data.room_id);
      break;
    case 'JoinedRoom':
      players = msg.data.players;
      document.getElementById('lobby-info').textContent =
        'Joueurs: ' + players.join(', ');
      break;
    case 'GameStarted':
      document.getElementById('lobby').style.display = 'none';
      initGame();
      setHand(msg.data.your_hand);
      yourTurn = (msg.data.turn === 0);
      break;
    case 'CardPlayed':
      onOpponentPlay(msg.data.player, msg.data.card_id);
      yourTurn = true;
      break;
    case 'CardDrawn':
      onOpponentDraw(msg.data.player, msg.data.card_id);
      yourTurn = true;
      break;
    case 'Error':
      alert('Erreur: ' + msg.data.msg);
      break;
  }
}

window.addEventListener('load', () => {
  initSocket(handleServerMessage);

  document.getElementById('btn-create').onclick = () => {
    send('CreateRoom', { name: 'Ma partie' });
  };
  document.getElementById('btn-join').onclick = () => {
    const room = document.getElementById('input-room').value;
    send('JoinRoom', { room_id: room });
  };
  document.getElementById('btn-start').onclick = () => {
    send('StartGame');
  };
});

export function playCard(cardId) {
  if (!yourTurn) return;
  send('PlayCard', { card_id: cardId });
}

export function drawCard() {
  if (!yourTurn) return;
  send('DrawCard');
}

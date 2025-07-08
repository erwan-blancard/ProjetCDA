const btnShowFriendList = document.getElementById('show-friend-list');
console.log("show-friend-list:", btnShowFriendList);
if (!btnShowFriendList) {
  console.error("Le bouton 'show-friend-list' est introuvable !");
}
const friendPanel = document.getElementById('friend-panel');
const friendListDiv = document.getElementById('friend-list');
const addFriendInput = document.getElementById('add-friend-name');
const addFriendButton = document.getElementById('add-friend-button');
const addFriendFeedback = document.getElementById('add-friend-feedback');
const closeFriendButton = document.getElementById('close-friend-button');
const friendPanelBackdrop = document.getElementById('friend-panel-backdrop');
import { get_friends, send_friend_request, get_friend_requests, respond_friend_request, delete_friend } from '../api/account.js';

let friends = [
  { username: "Bob", online: false },
  { username: "DarkVador66", online: true },
  { username: "Sp1derman", online: false },
  { username: "Fireball99", online: false },
  { username: "Mister_Vegeta", online: false },
  { username: "Anna_0509", online: false }
];

// Ajout d'une section pour les demandes d'amis
const friendRequestsDiv = document.createElement('div');
friendRequestsDiv.id = 'friend-requests-list';
friendRequestsDiv.style.marginBottom = '15px';

async function renderFriendList() {
  // Récupère la vraie liste d'amis depuis l'API
  let apiFriends = [];
  try {
    const res = await get_friends();
    if (Array.isArray(res)) {
      apiFriends = res.map(f => ({
        username: f.username || 'Inconnu',
        online: !!f.online,
        in_lobby: !!f.in_lobby
      }));
    }
  } catch (e) {
    console.error('Erreur lors de la récupération des amis:', e);
  }
  // Ajoute Bob à la liste
  const allFriends = [{ username: "Bob", online: false, in_lobby: false }, ...apiFriends];
  friendListDiv.innerHTML = '';
  // Trie : d'abord in_lobby, puis online, puis hors ligne
  const sorted = allFriends.sort((a, b) => {
    if (b.in_lobby !== a.in_lobby) return b.in_lobby - a.in_lobby;
    if (b.online !== a.online) return b.online - a.online;
    return a.username.localeCompare(b.username);
  });
  sorted.forEach((friend, index) => {
    const line = document.createElement('div');
    line.style.display = 'flex';
    line.style.justifyContent = 'space-between';
    line.style.alignItems = 'center';
    line.style.marginBottom = '5px';

    const info = document.createElement('span');
    info.textContent = `${friend.username} - `;
    // Couleur selon l'état
    if (friend.in_lobby) {
      info.textContent += 'Dans un lobby';
      if (friend.lobby_id) {
        info.textContent += ` (${friend.lobby_id})`;
      }
      info.style.color = '#90EE90';
      info.style.fontWeight = 'bold';
    } else if (friend.online) {
      info.textContent += 'En ligne';
      info.style.color = '#4ee44e'; 
      info.style.fontWeight = 'bold';
    } else {
      info.textContent += 'Hors ligne';
      info.style.color = 'gray';
      info.style.fontWeight = 'normal';
    }

    const removeButton = document.createElement('button');
    removeButton.textContent = 'X';
    removeButton.style.background = 'red';
    removeButton.style.color = 'white';
    removeButton.style.border = 'none';
    removeButton.style.padding = '4px 8px';
    removeButton.style.borderRadius = '5px';
    removeButton.style.cursor = 'pointer';
    removeButton.style.fontSize = '0.8em';

    removeButton.addEventListener('click', async () => {
      await delete_friend(friend.username);
      renderFriendList();
    });

    line.appendChild(info);
    line.appendChild(removeButton);
    friendListDiv.appendChild(line);
  });
}

async function renderFriendRequests() {
  friendRequestsDiv.innerHTML = '';
  let requests = [];
  try {
    requests = await get_friend_requests();
    if (!Array.isArray(requests)) requests = [];
  } catch (e) {
    console.error('Erreur lors de la récupération des demandes d\'amis:', e);
  }
  // Message de confirmation
  const confirmDiv = document.createElement('div');
  confirmDiv.id = 'friend-request-confirm';
  confirmDiv.style.color = 'green';
  confirmDiv.style.marginBottom = '5px';
  friendRequestsDiv.appendChild(confirmDiv);
  if (requests.length === 0) {
    friendRequestsDiv.innerHTML += '<div style="color:gray;">Aucune demande d\'ami en attente.</div>';
    // Cloche blanche si aucune demande
    const notifBtn = document.getElementById('show-notifications');
    if (notifBtn) {
      notifBtn.style.backgroundColor = '';
      notifBtn.style.color = '';
      notifBtn.style.border = '';
    }
    return;
  }
  // Limite à 10 demandes
  const maxRequests = 10;
  const limitedRequests = requests.slice(0, maxRequests);
  if (requests.length > maxRequests) {
    const infoMsg = document.createElement('div');
    infoMsg.style.color = 'orange';
    infoMsg.style.marginBottom = '5px';
    infoMsg.textContent = `Affichage des 10 premières demandes sur ${requests.length}. Veuillez traiter vos demandes.`;
    friendRequestsDiv.appendChild(infoMsg);
  }
  // Cloche orange si demandes
  const notifBtn = document.getElementById('show-notifications');
  if (notifBtn) {
    notifBtn.style.backgroundColor = '#FFD700';
    notifBtn.style.color = '#b36b00';
    notifBtn.style.border = '2px solid #b36b00';
  }
  limitedRequests.forEach(req => {
    const pseudo = req.sender_username || req.account1_username || req.sender_username || req.account1_pseudo || req.account1 || 'Inconnu';
    let statut = 'En attente';
    if (req.status === 1) statut = 'Accepté';
    else if (req.status === 2) statut = 'Refusé';
    const line = document.createElement('div');
    line.style.display = 'flex';
    line.style.justifyContent = 'space-between';
    line.style.alignItems = 'center';
    line.style.marginBottom = '5px';
    line.id = `friend-request-line-${req.id}`;
    const info = document.createElement('span');
    info.textContent = `${pseudo} - ${statut}`;
    const acceptBtn = document.createElement('button');
    acceptBtn.textContent = 'Accepter';
    acceptBtn.style.background = 'green';
    acceptBtn.style.color = 'white';
    acceptBtn.style.border = 'none';
    acceptBtn.style.padding = '4px 8px';
    acceptBtn.style.borderRadius = '5px';
    acceptBtn.style.cursor = 'pointer';
    acceptBtn.style.marginRight = '5px';
    const refuseBtn = document.createElement('button');
    refuseBtn.textContent = 'Refuser';
    refuseBtn.style.background = 'gray';
    refuseBtn.style.color = 'white';
    refuseBtn.style.border = 'none';
    refuseBtn.style.padding = '4px 8px';
    refuseBtn.style.borderRadius = '5px';
    refuseBtn.style.cursor = 'pointer';
    if (req.status !== 0) {
      acceptBtn.disabled = true;
      refuseBtn.disabled = true;
      acceptBtn.style.opacity = '0.5';
      refuseBtn.style.opacity = '0.5';
    }
    acceptBtn.onclick = async () => {
      await respond_friend_request(pseudo, true);
      confirmDiv.textContent = `Demande de ${pseudo} acceptée !`;
      setTimeout(() => { confirmDiv.textContent = ''; }, 2000);
      // Supprime la ligne du DOM immédiatement
      const lineElem = document.getElementById(`friend-request-line-${req.id}`);
      if (lineElem) lineElem.remove();
      await renderFriendList();
    };
    refuseBtn.onclick = async () => {
      await respond_friend_request(pseudo, false);
      confirmDiv.textContent = `Demande de ${pseudo} refusée.`;
      setTimeout(() => { confirmDiv.textContent = ''; }, 2000);
      // Supprime la ligne du DOM immédiatement
      const lineElem = document.getElementById(`friend-request-line-${req.id}`);
      if (lineElem) lineElem.remove();
      await renderFriendList();
    };
    const btns = document.createElement('span');
    btns.appendChild(acceptBtn);
    btns.appendChild(refuseBtn);
    line.appendChild(info);
    line.appendChild(btns);
    friendRequestsDiv.appendChild(line);
  });
}

btnShowFriendList.addEventListener('click', () => {
  const isHidden = friendPanel.style.display === 'none' || friendPanel.style.display === '';
  if (isHidden) {
    renderFriendRequests();
    renderFriendList();
    // Ajoute la section demandes d'amis en haut du panneau si pas déjà présente
    if (!friendPanel.contains(friendRequestsDiv)) {
      friendPanel.insertBefore(friendRequestsDiv, friendPanel.firstChild);
      // Optionnel: titre
      if (!document.getElementById('friend-requests-title')) {
        const title = document.createElement('div');
        title.id = 'friend-requests-title';
        title.textContent = 'Demandes d\'amis reçues';
        title.style.fontWeight = 'bold';
        title.style.marginBottom = '5px';
        friendPanel.insertBefore(title, friendRequestsDiv);
      }
    }
    friendPanel.style.display = 'block';
    friendPanelBackdrop.style.display = 'block';
  } else {
    friendPanel.style.display = 'none';
    friendPanelBackdrop.style.display = 'none';
  }
});

closeFriendButton.addEventListener('click', () => {
  friendPanel.style.display = 'none';
  friendPanelBackdrop.style.display = 'none';
});

friendPanelBackdrop.addEventListener('click', () => {
  friendPanel.style.display = 'none';
  friendPanelBackdrop.style.display = 'none';
});

addFriendButton.addEventListener('click', async () => {
  const name = addFriendInput.value.trim();
  if (!name) {
    addFriendFeedback.textContent = "Entrez un nom valide.";
    addFriendFeedback.style.color = 'red';
    return;
  }
  // Vérifie si déjà dans la liste affichée
  const currentFriends = Array.from(friendListDiv.children).map(div => div.firstChild.textContent.split(' - ')[0]);
  if (currentFriends.some(f => f.toLowerCase() === name.toLowerCase())) {
    addFriendFeedback.textContent = "Cet ami est déjà dans la liste.";
    addFriendFeedback.style.color = 'red';
    return;
  }
  // Appel API pour envoyer la demande
  addFriendFeedback.textContent = "Envoi de la demande...";
  addFriendFeedback.style.color = 'black';
  const result = await send_friend_request(name);
  if (result && result.id) {
    addFriendFeedback.textContent = "Demande envoyée à " + name + " !";
    addFriendFeedback.style.color = 'green';
    addFriendInput.value = '';
    // Change la couleur du bouton cloche
    const notifBtn = document.getElementById('show-notifications');
    if (notifBtn) {
      notifBtn.style.backgroundColor = '#FFD700'; // jaune-orangé
      notifBtn.style.color = '#b36b00';
      notifBtn.style.border = '2px solid #b36b00';
    }
  } else {
    addFriendFeedback.textContent = "Erreur : " + (result && result.message ? result.message : "Ce pseudo n'existe pas ou la demande a échoué.");
    addFriendFeedback.style.color = 'red';
  }
});

const buttons = document.getElementsByTagName('button');
for (let i = 0; i < buttons.length; i++) {
  buttons[i].setAttribute("tabindex", "-1");
}

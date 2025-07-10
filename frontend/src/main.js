import { get_friend_requests } from './js/api/account.js';
import { get_my_account } from './js/api/account.js';
import { displayPseudo } from './js/ui/friend_panel.js';

function updateFriendButtonColor(requests) {
  const btnShowFriendList = document.getElementById('show-friend-list');
  if (!btnShowFriendList) return;
  btnShowFriendList.style.backgroundColor = 'white';
  btnShowFriendList.style.color = 'black';
  if (requests && requests.length > 0) {
    btnShowFriendList.style.backgroundColor = '#d6ff70'; // jaune-vert
    btnShowFriendList.style.color = '#5a5a00';
  }
}

async function refreshFriendButtonColor() {
  // On attend que le bouton soit dans le DOM
  let tries = 0;
  function tryUpdate() {
    const btn = document.getElementById('show-friend-list');
    if (btn) {
      get_friend_requests().then(updateFriendButtonColor);
    } else if (tries < 10) {
      tries++;
      setTimeout(tryUpdate, 200); // réessaye dans 200ms
    }
  }
  tryUpdate();
}

async function updateWelcomeBar() {
  const welcomeUser = document.getElementById('welcome-username');
  if (!welcomeUser) return;
  try {
    const account = await get_my_account();
    console.log('Compte récupéré via /account/profile :', account);
    if (account && account.username) {
      alert('Pseudo récupéré : ' + account.username);
      welcomeUser.textContent = account.username;
    } else {
      welcomeUser.textContent = '';
    }
  } catch (e) {
    welcomeUser.textContent = '';
  }
}

window.addEventListener('DOMContentLoaded', () => {
  displayPseudo('main-username');

  // Ajout panel paramètres
  let settingsPanel = document.getElementById('settings-panel');
  let settingsPanelBackdrop = document.getElementById('settings-panel-backdrop');
  if (!settingsPanel) {
    settingsPanel = document.createElement('div');
    settingsPanel.id = 'settings-panel';
    settingsPanel.style.display = 'none';
    settingsPanel.style.position = 'fixed';
    settingsPanel.style.top = '50%';
    settingsPanel.style.left = '50%';
    settingsPanel.style.transform = 'translate(-50%, -50%)';
    settingsPanel.style.background = 'white';
    settingsPanel.style.border = '1px solid #ccc';
    settingsPanel.style.borderRadius = '10px';
    settingsPanel.style.padding = '24px 32px';
    settingsPanel.style.zIndex = '100';
    settingsPanel.innerHTML = '<h2>Paramètres</h2><div style="margin-top:1em;">(À venir)</div><button id="close-settings-panel" class="styled" style="margin-top:2em;">Fermer</button>';
    document.body.appendChild(settingsPanel);
  }
  if (!settingsPanelBackdrop) {
    settingsPanelBackdrop = document.createElement('div');
    settingsPanelBackdrop.id = 'settings-panel-backdrop';
    settingsPanelBackdrop.style.display = 'none';
    settingsPanelBackdrop.style.position = 'fixed';
    settingsPanelBackdrop.style.top = '0';
    settingsPanelBackdrop.style.left = '0';
    settingsPanelBackdrop.style.right = '0';
    settingsPanelBackdrop.style.bottom = '0';
    settingsPanelBackdrop.style.width = '100vw';
    settingsPanelBackdrop.style.height = '100vh';
    settingsPanelBackdrop.style.background = 'rgba(0,0,0,0.3)';
    settingsPanelBackdrop.style.zIndex = '99';
    document.body.appendChild(settingsPanelBackdrop);
  }
  const paramBtn = document.getElementById('settings-btn');
  if (paramBtn) {
    paramBtn.addEventListener('click', () => {
      document.getElementById('settings-menu').style.display = 'none';
      settingsPanel.style.display = 'block';
      settingsPanelBackdrop.style.display = 'block';
    });
  }
  settingsPanelBackdrop.addEventListener('click', () => {
    settingsPanel.style.display = 'none';
    settingsPanelBackdrop.style.display = 'none';
  });
  settingsPanel.addEventListener('click', (e) => {
    if (e.target.id === 'close-settings-panel') {
      settingsPanel.style.display = 'none';
      settingsPanelBackdrop.style.display = 'none';
    }
  });

  // Gestion du menu paramètres/déconnexion
  const settingsBtn = document.getElementById('show-settings-menu');
  const settingsMenu = document.getElementById('settings-menu');
  const logoutBtn = document.getElementById('logout-btn');
  const paramBtn2 = document.getElementById('settings-btn'); // Renommé pour éviter la confusion
  if (paramBtn2) {
    paramBtn2.addEventListener('click', () => {
      settingsMenu.style.display = 'none';
      // Ouvre le panel paramètres (pour l'instant, alert)
      alert('Panel paramètres à implémenter !');
    });
  }
  if (logoutBtn) {
    logoutBtn.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Bouton déconnexion en bas à droite
  const logoutFab = document.getElementById('logout-fab');
  if (logoutFab) {
    logoutFab.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Bouton déconnexion latéral
  const logoutSide = document.getElementById('logout-side');
  if (logoutSide) {
    logoutSide.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Panel menu burger (panel central)
  let burgerPanel = document.getElementById('burger-panel');
  let burgerPanelBackdrop = document.getElementById('burger-panel-backdrop');
  if (!burgerPanel) {
    burgerPanel = document.createElement('div');
    burgerPanel.id = 'burger-panel';
    burgerPanel.style.display = 'none';
    burgerPanel.style.position = 'fixed';
    burgerPanel.style.top = '50%';
    burgerPanel.style.left = '50%';
    burgerPanel.style.transform = 'translate(-50%, -50%)';
    burgerPanel.style.background = 'white';
    burgerPanel.style.border = '1px solid #ccc';
    burgerPanel.style.borderRadius = '10px';
    burgerPanel.style.padding = '32px 40px';
    burgerPanel.style.zIndex = '100';
    burgerPanel.style.display = 'none';
    burgerPanel.innerHTML = '<button id="close-burger-panel" class="styled" style="font-size: 1.2em;">Retour</button>';
    document.body.appendChild(burgerPanel);
  }
  if (!burgerPanelBackdrop) {
    burgerPanelBackdrop = document.createElement('div');
    burgerPanelBackdrop.id = 'burger-panel-backdrop';
    burgerPanelBackdrop.style.display = 'none';
    burgerPanelBackdrop.style.position = 'fixed';
    burgerPanelBackdrop.style.top = '0';
    burgerPanelBackdrop.style.left = '0';
    burgerPanelBackdrop.style.right = '0';
    burgerPanelBackdrop.style.bottom = '0';
    burgerPanelBackdrop.style.width = '100vw';
    burgerPanelBackdrop.style.height = '100vh';
    burgerPanelBackdrop.style.background = 'rgba(0,0,0,0.3)';
    burgerPanelBackdrop.style.zIndex = '99';
    document.body.appendChild(burgerPanelBackdrop);
  }
  const burgerBtn = document.getElementById('show-settings-menu');
  if (burgerBtn) {
    burgerBtn.addEventListener('click', () => {
      // Ferme le menu déroulant s'il est ouvert
      if (settingsMenu) settingsMenu.style.display = 'none';
      // Affiche le panel central
      burgerPanel.style.display = 'block';
      burgerPanelBackdrop.style.display = 'block';
      // S'assure que le z-index est bien supérieur
      burgerPanel.style.zIndex = '200';
      burgerPanelBackdrop.style.zIndex = '199';
    });
  }
  document.body.addEventListener('click', (e) => {
    if (e.target.id === 'close-burger-panel' || e.target.id === 'burger-panel-backdrop') {
      burgerPanel.style.display = 'none';
      burgerPanelBackdrop.style.display = 'none';
    }
  });
});
setTimeout(updateWelcomeBar, 1000); 
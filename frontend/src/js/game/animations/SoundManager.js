/**
 * Gestionnaire des effets sonores du jeu
 * Gère la lecture des sons selon les actions
 */
export class SoundManager {
    constructor() {
        this.sounds = new Map();
        this.audioContext = null;
        this.soundFiles = new Map();
        this.initAudioContext();
        this.preloadSounds();
    }

    /**
     * Initialise le contexte audio
     */
    initAudioContext() {
        try {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        } catch (error) {
            console.warn('AudioContext non supporté:', error);
        }
    }

    /**
     * Précharge les fichiers audio
     */
    preloadSounds() {
        // Déterminer le chemin de base selon l'environnement
        const basePath = this.getBasePath();
        console.log(`🔗 Chemin de base détecté: ${basePath}`);
        
        // Sons d'attaque (utilise les vrais fichiers disponibles)
        this.loadSound('attack_light', `${basePath}/src/public/assets/sounds/attacks/hit_sfx.wav`);
        this.loadSound('attack_medium', `${basePath}/src/public/assets/sounds/attacks/hit_sfx.wav`);
        this.loadSound('attack_heavy', `${basePath}/src/public/assets/sounds/attacks/hit_sfx.wav`);
        this.loadSound('water_attack', `${basePath}/src/public/assets/sounds/attacks/water_sfx.wav`);
        
        // Sons de soin (utilise le vrai fichier de soin)
        this.loadSound('heal_light', `${basePath}/src/public/assets/sounds/heals/heal_sfx.wav`);
        this.loadSound('heal_strong', `${basePath}/src/public/assets/sounds/heals/heal_sfx.wav`);
        
        // Sons de cartes (utilise un fichier qui fonctionne)
        this.loadSound('draw', `${basePath}/src/public/assets/sounds/dice/dice_roll_sfx.wav`); // Utilise le son de dé temporairement
        this.loadSound('discard', `${basePath}/src/public/assets/sounds/dice/dice_roll_sfx.wav`); // Utilise le son de dé temporairement
        this.loadSound('play_card', `${basePath}/src/public/assets/sounds/dice/dice_roll_sfx.wav`); // Utilise le son de dé temporairement
        
        // Sons de dés (utilise le vrai fichier de dé)
        this.loadSound('dice_roll', `${basePath}/src/public/assets/sounds/dice/dice_roll_sfx.wav`);
        this.loadSound('dice_land', `${basePath}/src/public/assets/sounds/dice/dice_roll_sfx.wav`); // Réutilise le son de dé pour l'atterrissage
        
        // Sons d'interface (utilise les vrais fichiers UI)
        this.loadSound('victory', `${basePath}/src/public/assets/sounds/ui/victory_sfx.mp3`);
        this.loadSound('defeat', `${basePath}/src/public/assets/sounds/ui/lose_sfx.mp3`);
        this.loadSound('button_click', `${basePath}/src/public/assets/sounds/ui/victory_sfx.mp3`); // Réutilise le son de victoire pour les clics
        this.loadSound('notification', `${basePath}/src/public/assets/sounds/ui/victory_sfx.mp3`); // Réutilise le son de victoire pour les notifications
    }
    
    /**
     * Détermine le chemin de base selon l'environnement
     * @returns {string} Chemin de base
     */
    getBasePath() {
        // Si on est sur localhost ou un serveur de développement
        if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
            return ''; // Chemin relatif depuis la racine
        }
        
        // Si on est sur un serveur de production
        return window.location.origin;
    }

    /**
     * Charge un fichier audio
     * @param {string} name - Nom du son
     * @param {string} url - URL du fichier
     */
    loadSound(name, url) {
        const audio = new Audio();
        audio.preload = 'auto';
        audio.src = url;
        
        // Gestion des erreurs
        audio.addEventListener('error', (e) => {
            console.error(`❌ Erreur de chargement du son: ${name} (${url})`, e);
            console.error(`Type d'erreur:`, e.type);
            console.error(`Code d'erreur:`, audio.error?.code);
            console.error(`Message d'erreur:`, audio.error?.message);
        });
        
        // Gestion du succès
        audio.addEventListener('canplaythrough', () => {
            console.log(`✅ Son chargé avec succès: ${name} (${url})`);
        });
        
        // Gestion du chargement
        audio.addEventListener('loadstart', () => {
            console.log(`🔄 Chargement du son: ${name} (${url})`);
        });
        
        this.soundFiles.set(name, audio);
    }

    /**
     * Joue un son pour une action de carte
     * @param {Object} action - L'action de la carte
     */
    playCardSound(action) {
        switch (action.type) {
            case 'ATTACK':
                this.playAttackSound(action.amount);
                break;
            case 'HEAL':
                this.playHealSound(action.amount);
                break;
            case 'DRAW':
                this.playDrawSound();
                break;
            case 'DISCARD':
                this.playDiscardSound();
                break;
            case 'BOOST':
                this.playBoostSound();
                break;
            case 'STEAL':
                this.playStealSound();
                break;
            default:
                this.playGenericSound();
        }
    }

    /**
     * Joue un son d'attaque
     * @param {number} amount - Montant des dégâts
     */
    playAttackSound(amount) {
        let soundName;
        if (amount <= 3) {
            soundName = 'attack_light';
        } else if (amount <= 7) {
            soundName = 'attack_medium';
        } else {
            soundName = 'attack_heavy';
        }
        this.playSound(soundName);
    }

    /**
     * Joue un son de soin
     * @param {number} amount - Montant du soin
     */
    playHealSound(amount) {
        const soundName = amount <= 5 ? 'heal_light' : 'heal_strong';
        this.playSound(soundName);
    }

    /**
     * Joue un son de pioche
     */
    playDrawSound() {
        this.playSound('draw');
    }

    /**
     * Joue un son de défausse
     */
    playDiscardSound() {
        this.playSound('discard');
    }

    /**
     * Joue un son générique
     */
    playGenericSound() {
        this.playSound('play_card');
    }

    /**
     * Joue un fichier audio
     * @param {string} soundName - Nom du son à jouer
     */
    playSound(soundName) {
        console.log(`🎵 Tentative de lecture du son: ${soundName}`);
        const audio = this.soundFiles.get(soundName);
        
        if (audio) {
            console.log(`📁 Fichier audio trouvé pour: ${soundName}`);
            console.log(`🔗 URL du fichier:`, audio.src);
            console.log(`📊 État du fichier:`, {
                readyState: audio.readyState,
                networkState: audio.networkState,
                error: audio.error
            });
            
            // Cloner l'audio pour permettre les lectures simultanées
            const audioClone = audio.cloneNode();
            audioClone.volume = 0.7; // Volume par défaut
            
            audioClone.play().then(() => {
                console.log(`✅ Son ${soundName} joué avec succès`);
            }).catch(error => {
                console.error(`❌ Erreur lors de la lecture du son ${soundName}:`, error);
                console.error(`Type d'erreur:`, error.name);
                console.error(`Message:`, error.message);
                // Fallback vers les sons synthétiques
                console.log(`🔄 Utilisation du fallback synthétique pour: ${soundName}`);
                this.playSyntheticFallback(soundName);
            });
        } else {
            console.error(`❌ Son non trouvé: ${soundName}`);
            console.log(`📋 Sons disponibles:`, Array.from(this.soundFiles.keys()));
            // Fallback vers les sons synthétiques
            console.log(`🔄 Utilisation du fallback synthétique pour: ${soundName}`);
            this.playSyntheticFallback(soundName);
        }
    }

    /**
     * Fallback vers les sons synthétiques si les fichiers audio ne sont pas disponibles
     * @param {string} soundName - Nom du son
     */
    playSyntheticFallback(soundName) {
        if (!this.audioContext) return;

        switch (soundName) {
            case 'attack_light':
                this.playTone(200, 0.1, 'sawtooth');
                break;
            case 'attack_medium':
                this.playTone(150, 0.15, 'sawtooth');
                break;
            case 'attack_heavy':
                this.playTone(100, 0.2, 'sawtooth');
                break;
            case 'heal_light':
                this.playTone(400, 0.3, 'sine');
                break;
            case 'heal_strong':
                this.playTone(500, 0.4, 'sine');
                break;
            case 'draw':
                this.playTone(300, 0.2, 'triangle');
                break;
            case 'discard':
                this.playTone(150, 0.15, 'square');
                break;
            case 'play_card':
                this.playTone(250, 0.1, 'sine');
                break;
            default:
                this.playTone(250, 0.1, 'sine');
        }
    }

    /**
     * Joue un ton synthétique
     * @param {number} frequency - Fréquence en Hz
     * @param {number} duration - Durée en secondes
     * @param {string} type - Type d'onde (sine, square, sawtooth, triangle)
     */
    playTone(frequency, duration, type = 'sine') {
        if (!this.audioContext) return;

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        oscillator.frequency.setValueAtTime(frequency, this.audioContext.currentTime);
        oscillator.type = type;

        // Enveloppe ADSR simple
        const now = this.audioContext.currentTime;
        gainNode.gain.setValueAtTime(0, now);
        gainNode.gain.linearRampToValueAtTime(0.3, now + 0.01); // Attack
        gainNode.gain.linearRampToValueAtTime(0.1, now + duration * 0.7); // Decay
        gainNode.gain.linearRampToValueAtTime(0.1, now + duration * 0.9); // Sustain
        gainNode.gain.linearRampToValueAtTime(0, now + duration); // Release

        oscillator.start(now);
        oscillator.stop(now + duration);
    }

    /**
     * Joue un son de dé
     * @param {number} result - Résultat du dé
     */
    playDiceSound(result) {
        if (!this.audioContext) return;

        // Son différent selon le résultat
        const frequency = 100 + (result * 50);
        const duration = 0.1;
        
        this.playTone(frequency, duration, 'square');
    }

    /**
     * Joue un son de victoire
     */
    playVictorySound() {
        if (!this.audioContext) return;

        // Mélodie de victoire
        const notes = [523.25, 659.25, 783.99]; // C5, E5, G5
        notes.forEach((freq, index) => {
            setTimeout(() => {
                this.playTone(freq, 0.3, 'sine');
            }, index * 200);
        });
    }

    /**
     * Joue un son de défaite
     */
    playDefeatSound() {
        this.playSound('defeat');
    }

    /**
     * Joue un son de victoire
     */
    playVictorySound() {
        this.playSound('victory');
    }

    /**
     * Joue un son d'attaque d'eau
     */
    playWaterAttackSound() {
        this.playSound('water_attack');
    }

    /**
     * Joue un son de dé
     */
    playDiceSound() {
        this.playSound('dice_roll');
    }

    /**
     * Joue un son de boost
     */
    playBoostSound() {
        // Utilise le son de victoire pour le boost (temporaire)
        this.playSound('victory');
    }

    /**
     * Joue un son de vol
     */
    playStealSound() {
        // Utilise le son de pioche pour le vol (temporaire)
        this.playSound('draw');
    }
}

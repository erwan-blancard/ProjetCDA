import * as THREE from 'three';
import { EffectGenerator } from './EffectGenerator.js';
import { SoundManager } from './SoundManager.js';

/**
 * Gestionnaire central des animations du jeu
 * Coordonne les effets visuels et sonores
 */
export class AnimationManager {
    constructor(scene, camera = null) {
        this.scene = scene;
        this.camera = camera;
        this.effectGenerator = new EffectGenerator(scene);
        this.soundManager = new SoundManager();
        this.activeEffects = new Set();
    }
    
    /**
     * Définit la caméra pour les effets de shake screen
     * @param {THREE.Camera} camera - La caméra à utiliser
     */
    setCamera(camera) {
        this.camera = camera;
    }

    /**
     * Lance l'effet d'une carte jouée
     * @param {Object} card - La carte jouée
     * @param {Object} target - La cible (peut être null)
     * @param {Object} action - L'action de la carte
     */
    playCardEffect(card, target, action) {
        console.log('🎬 Animation: Playing card effect', { card, target, action });
        
        // Étape 1: Générer l'effet au lancement de la carte
        const effect = this.effectGenerator.createCardEffect(card, action);
        if (effect) {
            this.activeEffects.add(effect);
            this.scene.add(effect);
        }

        // Étape 2: Déplacer l'effet vers la cible si c'est une attaque
        if (target && action.type === 'ATTACK' && action.amount > 0) {
            this.moveEffectToTarget(effect, target);
        }

        // Étape 3: Effet de shake screen pour les cartes sismiques
        const shakeConfig = this.effectGenerator.createScreenShakeEffect(action);
        if (shakeConfig && this.camera) {
            console.log('🌍 Effet de shake screen déclenché!', shakeConfig);
            this.effectGenerator.applyScreenShake(this.camera, shakeConfig);
        }

        // Étape 4: Créer un son
        this.soundManager.playCardSound(action);

        // Nettoyer l'effet après animation
        if (effect) {
            setTimeout(() => {
                this.cleanupEffect(effect);
            }, 2000); // 2 secondes par défaut
        }
    }

    /**
     * Déplace un effet vers une cible
     * @param {THREE.Object3D} effect - L'effet à déplacer
     * @param {Object} target - La cible
     */
    moveEffectToTarget(effect, target) {
        if (!effect || !target) return;

        // Position de départ (carte)
        const startPos = effect.position.clone();
        
        // Position d'arrivée (cible)
        const targetPos = new THREE.Vector3(
            target.position?.x || 0,
            target.position?.y || 0,
            target.position?.z || 0
        );

        // Animation de déplacement
        const tl = gsap.timeline();
        tl.to(effect.position, {
            x: targetPos.x,
            y: targetPos.y,
            z: targetPos.z,
            duration: 0.8,
            ease: "power2.out"
        });
    }

    /**
     * Nettoie un effet de la scène
     * @param {THREE.Object3D} effect - L'effet à nettoyer
     */
    cleanupEffect(effect) {
        if (effect && effect.parent) {
            effect.parent.remove(effect);
            this.activeEffects.delete(effect);
        }
    }

    /**
     * Nettoie tous les effets actifs
     */
    cleanupAllEffects() {
        this.activeEffects.forEach(effect => {
            this.cleanupEffect(effect);
        });
        this.activeEffects.clear();
    }
}

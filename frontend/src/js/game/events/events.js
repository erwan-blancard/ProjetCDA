import { Card, OpponentCard, getCardElement } from "../cards";
import { Opponent, Player, PlayerObject } from "../player";
import { EventMgr } from "./event_mgr";
import * as THREE from 'three';
import * as GAME from "../game";
import { GameStatusResponse } from "../../server/dto";
import gsap from "gsap";
import { sleep } from "../../utils";
import { AnimationManager } from "../animations/AnimationManager.js";


export class GameEvent {
    /**
     * @type {EventMgr | null}
     * set by EventMgr when pushed to the queue
     */
    mgr = null;
    timeout = 250;    // in ms
    #started = 0;

    constructor() {}

    // reimplement logic here
    async run() {}

    // read-only
    get started() { return this.#started; }

    /**
     * called by EventMgr
     * do not reimplement this function, reimplement run() instead
     */
    execute() {
        // if timeout < 0, it's up to run() to call notifyMgr() when done
        if (this.timeout >= 0)
            setTimeout(() => { this.onTimeout(); }, this.timeout);
        this.#started = Date.now();
        this.run();
    }

    // tells the EventMgr to execute next event in queue
    onTimeout() { this.mgr.executeNext(); }

}


export class PlayerEvent extends GameEvent {
    /** @type {PlayerObject | Player | Opponent} */
    player;

    constructor(player) {
        super();
        this.player = player;
    }
}

export class CardEvent extends GameEvent {
    /** @type {Card} */
    card;

    constructor(card) {
        super();
        this.card = card;
    }
}


export class DamagePlayerEvent extends PlayerEvent {

    constructor(player, amount, element = 'fire', cardId = null) {
        super(player);
        this.amount = amount;
        this.element = element;
        this.cardId = cardId;
        this.timeout = 1500; // 1.5 seconde pour l'animation complète
    }

    async run() {
        console.log(`💥 DamagePlayerEvent: ${this.amount} dégâts (${this.element}) sur ${this.player.name}`);
        console.log(`🃏 Card ID: ${this.cardId} - Debug pour effets spéciaux`);
        
        // Animation de dégâts avec élément
        if (GAME.animationManager) {
            const action = { 
                type: 'ATTACK', 
                amount: this.amount,
                element: this.element,
                cardId: this.cardId
            };
            
            // Déclencher l'effet de shake screen pour les cartes sismiques
            const shakeConfig = GAME.animationManager.effectGenerator.createScreenShakeEffect(action);
            if (shakeConfig && GAME.animationManager.camera) {
                console.log('🌍 Effet de shake screen déclenché!', shakeConfig);
                GAME.animationManager.effectGenerator.applyScreenShake(GAME.animationManager.camera, shakeConfig);
            }
            
            // Déclencher l'effet de teinte bleue pour les cartes d'eau
            const tintConfig = GAME.animationManager.effectGenerator.createWaterTintEffect(action);
            if (tintConfig) {
                console.log('💙 Effet de teinte bleue déclenché!', tintConfig);
                GAME.animationManager.effectGenerator.applyWaterTint(tintConfig);
            }
            
            // Position de départ : main du joueur qui attaque (côté opposé au joueur ciblé)
            let startPosition;
            if (this.player === GAME.PLAYER) {
                // Si on attaque le joueur, l'attaque vient des adversaires
                startPosition = new THREE.Vector3(3, 1, 0); // Côté adversaire
            } else {
                // Si on attaque un adversaire, l'attaque vient du joueur
                startPosition = new THREE.Vector3(-3, 1, 0); // Côté joueur
            }
            
            // Créer l'effet d'attaque
            const effect = GAME.animationManager.effectGenerator.createAttackEffect(startPosition, action);
            if (effect) {
                GAME.scene.add(effect);
                
                // Déplacer l'effet vers le joueur ciblé
                if (this.player.mesh) {
                    const targetPosition = this.player.mesh.position.clone();
                    targetPosition.y += 1; // Au-dessus du joueur
                    
                    gsap.to(effect.position, {
                        x: targetPosition.x,
                        y: targetPosition.y,
                        z: targetPosition.z,
                        duration: 0.8,
                        ease: "power2.out"
                    });
                }
                
                // Nettoyer l'effet après animation
                setTimeout(() => {
                    GAME.scene.remove(effect);
                }, 2000);
            }
            
            // Jouer le son d'attaque
            GAME.animationManager.soundManager.playAttackSound(this.amount);
        }
        
        // Appliquer les dégâts
        this.player.health -= this.amount;
        
        // Animation de secousse du joueur
        if (this.player.mesh) {
            const originalPosition = this.player.mesh.position.clone();
            
            gsap.timeline()
                .to(this.player.mesh.position, { 
                    x: originalPosition.x + 0.2, 
                    duration: 0.1,
                    ease: "power2.out"
                })
                .to(this.player.mesh.position, { 
                    x: originalPosition.x - 0.2, 
                    duration: 0.1,
                    ease: "power2.out"
                })
                .to(this.player.mesh.position, { 
                    x: originalPosition.x, 
                    duration: 0.1,
                    ease: "power2.out"
                });
        }
        
        // Mettre à jour l'UI du joueur
        if (this.player.updateHealthDisplay) {
            this.player.updateHealthDisplay();
        }
    }

}


export class HealPlayerEvent extends PlayerEvent {

    constructor(player, amount) {
        super(player);
        this.amount = amount;
        this.timeout = 1500; // 1.5 seconde pour l'animation complète
    }

    async run() {
        console.log(`💚 HealPlayerEvent: ${this.amount} HP sur ${this.player.name}`);
        
        // Animation de soin
        if (GAME.animationManager) {
            const action = { type: 'HEAL', amount: this.amount };
            
            // Position de départ : main du joueur qui soigne (même côté que le joueur ciblé)
            let startPosition;
            if (this.player === GAME.PLAYER) {
                // Si on soigne le joueur, le soin vient du joueur
                startPosition = new THREE.Vector3(-3, 2, 0); // Côté joueur
            } else {
                // Si on soigne un adversaire, le soin vient de l'adversaire
                startPosition = new THREE.Vector3(3, 2, 0); // Côté adversaire
            }
            
            // Créer l'effet de soin
            const effect = GAME.animationManager.effectGenerator.createHealEffect(startPosition, action);
            if (effect) {
                GAME.scene.add(effect);
                
                // Nettoyer l'effet après animation
                setTimeout(() => {
                    GAME.scene.remove(effect);
                }, 2000);
            }
            
            // Jouer le son de soin
            GAME.animationManager.soundManager.playHealSound(this.amount);
        }
        
        // Appliquer le soin
        this.player.health += this.amount;
        
        // Animation de lueur verte du joueur
        if (this.player.mesh && this.player.mesh.material) {
            const originalColor = this.player.mesh.material.color.clone();
            
            gsap.timeline()
                .to(this.player.mesh.material.color, { 
                    r: 0.2, 
                    g: 1.0, 
                    b: 0.2, 
                    duration: 0.3,
                    ease: "power2.out"
                })
                .to(this.player.mesh.material.color, { 
                    r: originalColor.r, 
                    g: originalColor.g, 
                    b: originalColor.b, 
                    duration: 0.3,
                    ease: "power2.out"
                });
        }
        
        // Mettre à jour l'UI du joueur
        if (this.player.updateHealthDisplay) {
            this.player.updateHealthDisplay();
        }
    }

}


export class ThrowDiceEvent extends PlayerEvent {

    constructor(player, result) {
        super(player);
        this.result = result;
        this.timeout = -1;
    }

    async run() {
        console.log(`🎲 ThrowDiceEvent: ${this.player.name} lance un dé (résultat: ${this.result})`);
        
        // Jouer le son de dé au début
        if (GAME.animationManager) {
            GAME.animationManager.soundManager.playDiceSound();
        }
        
        try {
            GAME.dice.setPlayerName(this.player.name);
        } catch (e) {
            console.log("Exception when setting dice player name:", e);
            GAME.dice.setPlayerName("N/A");
        }
        
        GAME.dice.appear();
        await GAME.dice.cycleTo(this.result);
        
        // Jouer un son différent selon le résultat
        if (GAME.animationManager) {
            GAME.animationManager.soundManager.playDiceSound(this.result);
        }
        
        await GAME.dice.disappear();
        this.onTimeout();
    }

}


export class DrawCardEvent extends PlayerEvent {

    constructor(player, card_id) {
        super(player);
        this.card_id = card_id;
        this.timeout = 1200; // Animation de pioche plus longue
    }

    async run() {
        console.log(`🎴 DrawCardEvent: ${this.player.name} pioche une carte`);
        
        // Animation de pioche
        if (GAME.animationManager) {
            const action = { type: 'DRAW', amount: 1 };
            
            // Position de départ (pile de cartes)
            const startPosition = GAME.cardPile ? 
                GAME.cardPile.position.clone().add(new THREE.Vector3(0, 1, 0)) :
                new THREE.Vector3(0, 2, 0);
            
            // Créer l'effet de pioche
            const effect = GAME.animationManager.effectGenerator.createDrawEffect(startPosition, action);
            if (effect) {
                GAME.scene.add(effect);
                
                // Déplacer l'effet vers le joueur
                if (this.player.mesh) {
                    const targetPosition = this.player.mesh.position.clone();
                    targetPosition.y += 1;
                    
                    gsap.to(effect.position, {
                        x: targetPosition.x,
                        y: targetPosition.y,
                        z: targetPosition.z,
                        duration: 0.8,
                        ease: "power2.out"
                    });
                }
                
                // Nettoyer l'effet après animation
                setTimeout(() => {
                    GAME.scene.remove(effect);
                }, 2000);
            }
            
            // Jouer le son de pioche
            GAME.animationManager.soundManager.playDrawSound();
        }

        if (this.player == GAME.PLAYER) {
            const card = new Card(this.card_id);
            this.player.addCard(card);
            // add card to scene
            GAME.scene.add(card);
            GAME.cardPile.count -= 1;
        } else if (this.player != null) {
            // opponent, card_id is -1
            this.player.setCardCount(this.player.cards.length + 1);
        }
    }

}

export class DiscardCardEvent extends PlayerEvent {

    constructor(player, card_index) {
        super(player);
        this.card_index = card_index;
        this.timeout = 1000; // Animation de défausse plus longue
    }

    async run() {
        console.log(`🗑️ DiscardCardEvent: ${this.player.name} défausse une carte`);
        
        // Animation de défausse
        if (GAME.animationManager) {
            const action = { type: 'DISCARD', amount: 1 };
            
            // Position de départ (main du joueur)
            const startPosition = this.player.mesh ? 
                this.player.mesh.position.clone().add(new THREE.Vector3(0, 1, 0)) :
                new THREE.Vector3(0, 2, 0);
            
            // Créer l'effet de défausse
            const effect = GAME.animationManager.effectGenerator.createDiscardEffect(startPosition, action);
            if (effect) {
                GAME.scene.add(effect);
                
                // Animation de descente vers la défausse
                gsap.to(effect.position, {
                    y: -2,
                    duration: 0.8,
                    ease: "power2.in"
                });
                
                // Nettoyer l'effet après animation
                setTimeout(() => {
                    GAME.scene.remove(effect);
                }, 1500);
            }
            
            // Jouer le son de défausse
            GAME.animationManager.soundManager.playDiscardSound();
        }

        if (this.player != null) {
            this.player.cards.splice(this.card_index, 1).forEach((card) => {
                this.player.discard_cards.push(card);
            });
            
            this.player.updateHandCardPositions();
            this.player.emitCardCountChange();
            this.player.updateDiscardCardPositions();
            this.player.emitDiscardCountChange();
        }
    }

}


// event to move card towards the center of the playing field
export class PutCardForward extends CardEvent {

    constructor(card, display_card_id=-1) {
        super(card);
        this.display_card_id = display_card_id;
        this.timeout = 1000; // Animation de jeu de carte
    }

    async run() {
        if (this.card != null) {
            console.log(`🎯 PutCardForward: Carte jouée (ID: ${this.display_card_id})`);
            
            this.card.active = true; // prevent position updates with updateHandCardPositions() and updateDiscardCardPositions()

            if (this.card instanceof OpponentCard) {
                // change display of OpponentCard to match the expected card's look
                this.card.displayCardAsFront(this.display_card_id);
                this.card.flipCard();
            }

            // Déterminer l'élément de la carte
            const cardElement = getCardElement(this.display_card_id);
            
            // Animation de lueur de la carte selon son élément
            if (GAME.animationManager && this.card.material && this.card.material.color) {
                const elementConfig = GAME.animationManager.effectGenerator.getElementConfig(cardElement);
                
                // Animation de lueur de la carte
                const originalColor = this.card.material.color.clone();
                gsap.timeline()
                    .to(this.card.material.color, {
                        r: elementConfig.color.r,
                        g: elementConfig.color.g,
                        b: elementConfig.color.b,
                        duration: 0.3,
                        ease: "power2.out"
                    })
                    .to(this.card.material.color, {
                        r: originalColor.r,
                        g: originalColor.g,
                        b: originalColor.b,
                        duration: 0.3,
                        ease: "power2.out"
                    });
            }

            const pos = this.card.position;
            let new_pos = new THREE.Vector3(pos.x, pos.y, pos.z);
            new_pos.z += (this.card instanceof OpponentCard ? 2 : -2);
            new_pos.y += 0.5;
            this.card.goto(new_pos.x, new_pos.y, new_pos.z, this.timeout / 1000.0);
            
            // Jouer le son de jeu de carte
            if (GAME.animationManager) {
                GAME.animationManager.soundManager.playGenericSound();
            }
        }
    }
}

export class PutCardInPile extends GameEvent {
    
    constructor(player, card) {
        super();
        this.timeout = 400;
        this.player = player;
        this.card = card;
    }

    async run() {
        if (this.card != null) {
            this.card.active = false;
            // transfer card to discard pile
            this.player.discard_cards.push(this.card);
            // remove card from hand if it's not fake
            const idx = this.player.cards.indexOf(this.card);
            if (idx != -1)
                this.player.cards.splice(idx, 1);
            this.player.updateHandCardPositions();
            this.player.emitCardCountChange();
            this.player.updateDiscardCardPositions();
            this.player.emitDiscardCountChange();
        }
    }
}


export class ChangeTurnEvent extends PlayerEvent {

    constructor(player, turn_end=0) {
        super(player);
        this.turn_end = turn_end;
        this.timeout = 150;
    }

    async run() { GAME.updateCurrentPlayerTurn(this.player, this.turn_end); }

}


export class PlayerBuffsUpdateEvent extends PlayerEvent {

    constructor(player, buffs) {
        super(player);
        this.buffs = buffs;
        this.timeout = 0;
    }

    async run() {
        GAME.buffTooltip.visible = false;
        if (this.player)
            this.player.updateBuffs(this.buffs);
    }
}


export class CollectDiscardCardsEvent extends GameEvent {
    constructor(cards_in_pile) {
        super();
        this.timeout = -1;
        this.cards_in_pile = cards_in_pile;
    }
    
    async run() {
        let cards_remain = true;
        while (cards_remain) {
            if (GAME.PLAYER.discard_cards.length > 0) {
                const tl = gsap.timeline();
                const card = GAME.PLAYER.discard_cards.pop();
                tl.to(card.position, { x: 0.0, y: 0.0, z: 0.0, duration: 0.5, onComplete: () => {
                    card.removeFromParent();
                    GAME.cardPile.count += 1;
                } });
            } else {
                cards_remain = false;
            }

            let cards_remain_for_opponents = 0;
            for (const opponent of GAME.OPPONENTS.values()) {
                if (opponent.discard_cards.length > 0) {
                    cards_remain_for_opponents += 1;
                    const tl = gsap.timeline();
                    const card = opponent.discard_cards.pop();
                    tl.to(card.position, { x: 0.0, y: 0.0, z: 0.0, duration: 0.5, onComplete: () => {
                        card.removeFromParent();
                        GAME.cardPile.count += 1;
                    } });
                }
            }

            cards_remain = GAME.PLAYER.discard_cards.length > 0 || cards_remain_for_opponents > 0;
            if (cards_remain)
                await sleep(100);
            else
                await sleep(550);   // wait 0.5 secs to complete animations
        }

        this.onTimeout();
    }

    onTimeout() {
        GAME.cardPile.count = this.cards_in_pile;
        this.mgr.executeNext();
    }

}


/**
 * Event for GameStatus updates
 */
export class GameUpdateEvent extends GameEvent {
    /** @param {GameStatusResponse} upd_data  */
    upd_data;

    constructor(data) {
        super();
        this.upd_data = data;
        // this.timeout = 0;
    }

    async run() {
        const data = this.upd_data;

        GAME.buffTooltip.visible = false;

        try {
            GAME.PLAYER.health = data.health;
            GAME.PLAYER.updateHandCards(data.cards);
            GAME.PLAYER.updateDiscardCards(data.discard_cards);
            GAME.PLAYER.updateBuffs(data.buffs);

            // update opponents
            data.opponents.forEach(opponent_data => {
                const opponent = GAME.OPPONENTS.get(opponent_data.player_id);
                if (opponent != null) {
                    opponent.health = opponent_data.health;
                    opponent.setCardCount(opponent_data.card_count);
                    opponent.updateDiscardCards(opponent_data.discard_cards);
                    opponent.updateBuffs(opponent_data.buffs);
                }
            });

            GAME.cardPile.count = data.cards_in_pile;

            GAME.updateCurrentPlayerTurn(GAME.getPlayerById(data.current_player_turn), data.current_player_turn_end);
        } catch (e) {
            console.log("Exception when handling game update data:", e);
        }
    }

}

export class GameEndEvent extends GameEvent {
    constructor(winner_id) {
        super();
        this.timeout = -1;
        this.winner_id = winner_id;
    }

    async run() {
        GAME.displayGameEndScreen(this.winner_id);
    }

}

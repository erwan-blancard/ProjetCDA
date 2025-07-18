

export class CardKind {
    static FOOD = "Food";
    static SPELL = "Spell";
    static WEAPON = "Weapon";
}


export class CardInfo {
    name;
    element;
    kind;
    stars;
    desc;
    attack;
    heal;
    draw;
    attack_modifier;
    heal_modifier;
    draw_modifier;

    constructor(data) {
        this.name = data.name;
        this.element = data.element;
        this.kind = data.kind;
        this.stars = data.stars;
        this.desc = data.desc;
        this.attack = data.attack != undefined ? data.attack : 0;
        this.heal = data.heal != undefined ? data.heal : 0;
        this.draw = data.draw != undefined ? data.draw : 0;
        this.attack_modifier = data.attack_modifier != undefined ? data.attack_modifier : null;
        this.heal_modifier = data.heal_modifier != undefined ? data.heal_modifier : null;
        this.draw_modifier = data.draw_modifier != undefined ? data.draw_modifier : null;
    }

    get color() {
        switch (this.kind) {
            case CardKind.SPELL:
                return "#8D9FD1";
            case CardKind.WEAPON:
                return "#A9A9A9";
            case CardKind.FOOD:
                return "#A9CA3F";
            default:
                return "#404060";
        }
    }

}

// Ajout d'une détection d'environnement de test
const isNode = typeof window === 'undefined' || typeof document === 'undefined';

export let CARD_DATABASE = new Map();

if (!isNode) {
    await fetch('/assets/cards.json')
        .then(response => response.json()) // Parse JSON
        .then(cardsDb => {
            for (let i = 0; i < cardsDb.length; i++) {
                CARD_DATABASE.set(i, new CardInfo(cardsDb[i]));
            }
        })
        .catch(error => console.error('Error loading card database:', error));
}

// test
window.CARD_DATABASE = CARD_DATABASE;
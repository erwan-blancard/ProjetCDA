

export class CardInfo {
    name;
    element;
    kind;
    stars;
    desc;

    constructor(data) {
        this.name = data.name;
        this.element = data.element;
        this.kind = data.kind;
        this.stars = data.stars;
        this.desc = data.desc;
    }

    get color() {
        switch (this.kind) {
            case "Spell":
                return "#8D9FD1";
            case "Weapon":
                return "#A9A9A9";
            case "Food":
                return "#A9CA3F";
            default:
                return "#404060";
        }
    }

}

let cards = new Map();

await fetch('/assets/cards.json')
    .then(response => response.json()) // Parse JSON
    .then(cardsDb => {
        for (let i = 0; i < cardsDb.length; i++) {
            cards.set(i, new CardInfo(cardsDb[i]));
        }
    })
    .catch(error => console.error('Error loading card database:', error));

export let CARD_DATABASE = cards;

// test
window.CARD_DATABASE = CARD_DATABASE;
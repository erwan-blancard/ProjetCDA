export class CardKind {
    static FOOD = "Food";
    static SPELL = "Spell";
    static WEAPON = "Weapon";
}


/**
 * Class used to hint who the user can target when playing a card.
*/
export class TargetType {
    static SINGLE = "Single";
    static MULTIPLE = "Multiple";   // not used
    static ALL = "All";

    // not in Rust, used to tell if the player should be hinted for selection when playing a card
    // TODO use boolean in card info
    static SELF = "Self";
    static SINGLE_AND_SELF = "SingleAndSelf";
    static MULTIPLE_AND_SELF = "MultipleAndSelf";
    static ALL_AND_SELF = "AllAndSelf";

    static priorityOf(target) {
        switch (target) {
            case TargetType.SELF: return 0;
            case TargetType.ALL: return 1;
            case TargetType.ALL_AND_SELF: return 2;
            case TargetType.SINGLE: return 3;
            case TargetType.SINGLE_AND_SELF: return 4;
            case TargetType.MULTIPLE: return 5;
            case TargetType.MULTIPLE_AND_SELF: return 6;
            default: return -1;
        }
    }

    static cmp(a, b) {
        const aVal = TargetType.priorityOf(a);
        const bVal = TargetType.priorityOf(b);
        return bVal > aVal ? b : a;
    }

    static withSelfHint(target_type) {
        switch (target_type) {
            case TargetType.SINGLE: return TargetType.SINGLE_AND_SELF;
            case TargetType.MULTIPLE: return TargetType.MULTIPLE_AND_SELF;
            case TargetType.ALL: return TargetType.ALL_AND_SELF;
            default: return target_type;
        }
    }

    static withoutSelfHint(target_type) {
        switch (target_type) {
            case TargetType.SINGLE_AND_SELF: return TargetType.SINGLE;
            case TargetType.MULTIPLE_AND_SELF: return TargetType.MULTIPLE;
            case TargetType.ALL_AND_SELF: return TargetType.ALL;
            default: return target_type;
        }
    }
}


export class CardInfo {
    name;
    element;
    kind;
    stars;
    desc;
    type;
    attack;
    heal;
    draw;
    attack_modifier;
    heal_modifier;
    draw_modifier;
    #targets;

    constructor(data) {
        this.name = data.name;
        this.element = data.element;
        this.kind = data.kind;
        this.stars = data.stars;
        this.desc = data.desc;
        this.type = data.type;  // Rust class name
        this.attack = data.attack != undefined ? data.attack : 0;
        this.heal = data.heal != undefined ? data.heal : 0;
        this.draw = data.draw != undefined ? data.draw : 0;
        this.attack_modifier = data.attack_modifier != undefined ? data.attack_modifier : null;
        this.heal_modifier = data.heal_modifier != undefined ? data.heal_modifier : null;
        this.draw_modifier = data.draw_modifier != undefined ? data.draw_modifier : null;
        this.#targets = this.#get_target_type(data);
    }

    #get_target_type(data) {
        let target_type = TargetType.SINGLE;

        switch (data.type) {
            case "MultiActionCard":
                const target_types = data.target_types != undefined ? data.target_types : [TargetType.SINGLE];

                console.log(target_types);

                if (target_types.length > 0) {
                    target_type = TargetType.SELF;  // lowest priority action
                    // get target type with highest "priority"
                    target_types.forEach(t => {
                        console.log(TargetType.cmp(target_type, t));
                        target_type = TargetType.cmp(target_type, t);
                    });
                    console.log(target_type);
                }

                // check if only heal and/or draw -> type is Self

                const attacks = data.attacks != undefined ? data.attacks.reduce((acc, curr) => { return acc + curr }, 0) : 0
                const heals = data.heals != undefined ? data.heals.reduce((acc, curr) => { return acc + curr }, 0) : 0
                const draws = data.draws != undefined ? data.draws.reduce((acc, curr) => { return acc + curr }, 0) : 0

                const has_attack_modifiers = data.attack_modifiers != undefined ? data.attack_modifiers.reduce((acc, curr) => { return acc + (curr != null ? 1 : 0) }, 0) > 0 : false;
                const has_heal_modifiers = data.heal_modifiers != undefined ? data.heal_modifiers.reduce((acc, curr) => { return acc + (curr != null ? 1 : 0) }, 0) > 0 : false;
                const has_draw_modifiers = data.draw_modifiers != undefined ? data.draw_modifiers.reduce((acc, curr) => { return acc + (curr != null ? 1 : 0) }, 0) > 0 : false;

                if (target_type == TargetType.SINGLE) {
                    // check if only heal and/or draw -> type is Self
                    if ((attacks == 0 && !has_attack_modifiers)
                        && ((heals > 0 || has_heal_modifiers)
                        || (draws > 0 || has_draw_modifiers))) {
                        target_type = TargetType.SELF;
                    }
                }
                break;
            case "PearthCard":
                return TargetType.SINGLE;
            case "TargetBothCard":
                target_type = data.targets != undefined ? data.targets : TargetType.SINGLE;
                target_type = TargetType.withSelfHint(target_type);
                break;
            default:
                target_type = data.targets != undefined ? data.targets : TargetType.SINGLE;
                break;
        }

        if (target_type == TargetType.SINGLE) {
            // check if only heal and/or draw -> type is Self
            if ((this.attack == 0 && this.attack_modifier == null)
                && ((this.heal > 0 || this.heal_modifier != null)
                || (this.draw > 0 || this.draw_modifier != null))) {
                target_type = TargetType.SELF;
            }
        }

        return target_type;
    }

    get targets() { return this.#targets; }

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

let cards = new Map();

await fetch('/assets/cards.json')
    .then(response => response.json()) // Parse JSON
    .then(cardsDb => {
        for (let i = 0; i < cardsDb.length; i++) {
            cards.set(i, new CardInfo(cardsDb[i]));
        }
    })
    .catch(error => console.error('Error loading card database:', error));

export let CARD_COLLECTION = cards;

// test
window.CARD_COLLECTION = CARD_COLLECTION;
window.TargetType = TargetType;
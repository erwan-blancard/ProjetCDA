import * as THREE from "three";
import gsap from "gsap";
import { clamp, randInt } from "three/src/math/MathUtils.js";

const textureLoader = new THREE.TextureLoader();
const cardGeo = new THREE.BoxGeometry(1, 1.5, 0.001);
const cardCover = textureLoader.load("assets/randomi_verso.jpg");
cardCover.colorSpace = THREE.SRGBColorSpace;

export function getCardTexturePathById(id) {
  const number = id.toString().padStart(4, "0");
  return `assets/randomi_recto_cards_page-${number}.jpg`;
}

export class CardPile extends THREE.Mesh {
  static max_visible_cards = 25;
  static card_thickness = 0.05;
  #count;
  constructor() {
    const geo = new THREE.BoxGeometry(
      1,
      CardPile.card_thickness * CardPile.max_visible_cards,
      1.5
    );
    const mats = [
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial({ map: cardCover }),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial()
    ];
    super(geo, mats);
    this.#count = CardPile.max_visible_cards;
    this.#updatePile();
  }
  get count() {
    return this.#count;
  }
  set count(v) {
    this.#count = v;
    this.#updatePile();
  }
  drawCard() {
    const card_id = randInt(1, 100);
    this.count -= 1;
    return new Card(getCardTexturePathById(card_id));
  }
  #updatePile() {
    const h = CardPile.card_thickness * clamp(this.#count, 0, CardPile.max_visible_cards);
    this.position.y = -(CardPile.card_thickness * CardPile.max_visible_cards) / 2 + h;
    this.visible = this.#count > 0;
  }
}

export class Card extends THREE.Mesh {
  flipped = false;
  swingTimeline = gsap.timeline();
  constructor(image) {
    const tex = textureLoader.load(image);
    tex.colorSpace = THREE.SRGBColorSpace;
    const mats = [
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial(),
      new THREE.MeshBasicMaterial({ map: tex }),
      new THREE.MeshBasicMaterial({ map: cardCover })
    ];
    super(cardGeo, mats);
  }
  flipCard() {
    const tl = gsap.timeline();
    tl.to(this.rotation, { y: this.flipped ? 0 : -Math.PI, duration: 0.5 });
    this.flipped = !this.flipped;
  }
  startSwingLoop() {
    const angle = THREE.MathUtils.degToRad(4);
    this.swingTimeline.fromTo(
      this.rotation,
      { z: -angle },
      { z: angle, repeat: -1, duration: 1, yoyo: true }
    );
  }
  stopSwingLoop() {
    this.swingTimeline.clear();
    this.rotation.z = 0;
  }
}
import * as THREE from "three";
import { getCardTexturePathById, CardPile, Card } from "../cards";

jest.mock("three/src/math/MathUtils.js", () => ({
  clamp: (v, min, max) => Math.max(min, Math.min(max, v)),
  randInt: () => 42
}));

describe("cards.js", () => {
  test("getCardTexturePathById formatte l'ID", () => {
    expect(getCardTexturePathById(7)).toBe(
      "assets/randomi_recto_cards_page-0007.jpg"
    );
    expect(getCardTexturePathById(123)).toBe(
      "assets/randomi_recto_cards_page-0123.jpg"
    );
  });

  test("CardPile initial", () => {
    const pile = new CardPile();
    expect(pile.count).toBe(CardPile.max_visible_cards);
    expect(pile.visible).toBe(true);
  });

  test("CardPile.drawCard diminue count et renvoie Card", () => {
    const pile = new CardPile();
    const before = pile.count;
    const card = pile.drawCard();
    expect(pile.count).toBe(before - 1);
    expect(card).toBeInstanceOf(Card);
  });

  test("Card.flipCard bascule flipped", () => {
    const card = new Card("path");
    expect(card.flipped).toBe(false);
    card.flipCard();
    expect(card.flipped).toBe(true);
    card.flipCard();
    expect(card.flipped).toBe(false);
  });

  test("Card swing et stopSwingLoop remettent rotation.z à 0", () => {
    const card = new Card("path");
    card.startSwingLoop();
    expect(card.swingTimeline.getChildren().length).toBeGreaterThan(0);
    card.stopSwingLoop();
    expect(card.rotation.z).toBe(0);
  });
});

import { describe, it, expect } from 'vitest';
import { getCardTexturePathById } from './game/cards.js';

describe('getCardTexturePathById', () => {
  it('génère le bon chemin pour tous les ids de 1 à 100', () => {
    for (let id = 1; id <= 100; id++) {
      const path = getCardTexturePathById(id);
      expect(typeof path).toBe('string');
      expect(path).toBe(`assets/randomi_recto_cards_page-${id.toString().padStart(4, "0")}.jpg`);
    }
  });
}); 
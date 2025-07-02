import { describe, it, expect } from 'vitest';
import { collectViews } from './viewmgr.js';

describe('collectViews', () => {
  it('retourne un tableau', () => {
    const views = collectViews();
    expect(Array.isArray(views)).toBe(true);
  });
}); 
import { describe, it, expect } from 'vitest';
import { login_submit } from './auth.js';

describe('login_submit', () => {
  it('est une fonction', () => {
    expect(typeof login_submit).toBe('function');
  });
}); 
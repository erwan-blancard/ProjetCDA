// frontend/test/test_E2E/game.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Flux de jeu Randomi', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5174');
  });

  test('sélection de carte et bouton fin de tour', async ({ page }) => {
    await expect(page.locator('canvas')).toBeVisible();

    const { x, y, width, height } = await page.locator('canvas').boundingBox();
    await page.mouse.click(x + width/2 - 50, y + height/2 + 20);

    const [msg] = await Promise.all([
      page.waitForEvent('console'),
      page.mouse.click(x + width/2, y + height - 30)
    ]);
    expect(msg.text()).toContain('Tour terminé');
  });
});

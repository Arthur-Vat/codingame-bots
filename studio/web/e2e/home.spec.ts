import { expect, test } from '@playwright/test';

test('the home screen asks to choose a game', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('studio');
  await expect(page.getByRole('heading', { name: 'Choose a game' })).toBeVisible();
  await expect(page.getByText('Games appear here when the server is running.')).toBeVisible();
});

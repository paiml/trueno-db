import { test, expect } from '@playwright/test';

test.describe('trueno-db WASM Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for WASM to initialize (status shows version after init)
    await page.waitForFunction(() => {
      return document.getElementById('status')?.textContent?.includes('trueno-db-wasm');
    }, { timeout: 15000 });
  });

  test('loads and displays status', async ({ page }) => {
    const status = page.locator('#status');
    await expect(status).toBeVisible();
    const text = await status.textContent();
    expect(text).toContain('trueno-db-wasm');
  });

  test('detects compute tier', async ({ page }) => {
    const capabilities = page.locator('#capabilities');
    await expect(capabilities).toBeVisible();

    // Wait for tier detection to complete
    await page.waitForFunction(() => {
      const el = document.getElementById('capabilities');
      return el && !el.textContent?.includes('Detecting');
    }, { timeout: 10000 });

    const text = await capabilities.textContent();
    // Should show one of: WEBGPU, SIMD128, or SCALAR
    expect(text).toMatch(/(WEBGPU|SIMD128|SCALAR)/);
  });

  test('shows WebGPU or SIMD128 status indicators', async ({ page }) => {
    const capabilities = page.locator('#capabilities');

    // Wait for detection
    await page.waitForFunction(() => {
      const el = document.getElementById('capabilities');
      return el && el.textContent?.includes('WebGPU:');
    }, { timeout: 10000 });

    const text = await capabilities.textContent();
    // Should show checkmarks or X marks for capabilities
    expect(text).toMatch(/WebGPU: [✅❌]/);
    expect(text).toMatch(/SIMD128: [✅❌]/);
  });

  test('sql textarea is editable', async ({ page }) => {
    const sql = page.locator('#sql');
    await expect(sql).toBeVisible();

    // Clear and type new query
    await sql.fill('SELECT id, name FROM users WHERE active = true LIMIT 5');
    const value = await sql.inputValue();
    expect(value).toContain('SELECT id, name FROM users');
  });

  test('execute query button shows result', async ({ page }) => {
    const executeBtn = page.getByRole('button', { name: 'Execute Query' });
    await expect(executeBtn).toBeVisible();

    await executeBtn.click();

    // Wait for results to appear
    await page.waitForTimeout(500);

    const results = page.locator('#results');
    const text = await results.textContent();
    // Should show some result (could be success or error)
    expect(text).toBeTruthy();
    expect(text?.length).toBeGreaterThan(0);
  });

  test('load demo button triggers load', async ({ page }) => {
    const loadBtn = page.getByRole('button', { name: 'Load Demo Data' });
    await expect(loadBtn).toBeVisible();

    await loadBtn.click();

    // Wait for loading message
    await page.waitForTimeout(500);

    const results = page.locator('#results');
    const text = await results.textContent();
    // Should show loading or result message
    expect(text).toBeTruthy();
  });

  test('page has correct title', async ({ page }) => {
    await expect(page).toHaveTitle('Trueno-DB Browser Demo');
  });

  test('architecture diagram is visible', async ({ page }) => {
    const pre = page.locator('pre');
    await expect(pre.first()).toBeVisible();

    // Check for architecture text
    const text = await page.content();
    expect(text).toContain('Browser → WASM → trueno-db');
    expect(text).toContain('QueryEngine');
    expect(text).toContain('Arrow Tables');
  });

  test('screenshot: initial state', async ({ page }) => {
    // Wait for full initialization
    await page.waitForTimeout(1000);

    await expect(page).toHaveScreenshot('demo-initial.png', {
      fullPage: true,
      animations: 'disabled',
      maxDiffPixelRatio: 0.10,
    });
  });

  test('screenshot: after query', async ({ page }) => {
    const executeBtn = page.getByRole('button', { name: 'Execute Query' });
    await executeBtn.click();
    await page.waitForTimeout(500);

    await expect(page).toHaveScreenshot('demo-after-query.png', {
      fullPage: true,
      animations: 'disabled',
      maxDiffPixelRatio: 0.10,
    });
  });
});

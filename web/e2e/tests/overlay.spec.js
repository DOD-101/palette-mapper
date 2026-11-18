import { expect, test } from "@playwright/test";

import {
  closeOverlay,
  convert,
  gotoApp,
  selectPalette,
  uploadImage,
} from "../helpers.js";

test("overlay closes only when clicking outside the image", async ({ page }) => {
  await gotoApp(page);
  await selectPalette(page, "tokyonightstorm");
  await uploadImage(page);
  await convert(page);

  const backdrop = page.locator(".backdrop");
  const image = page.locator(".img-preview-btn");

  await expect(backdrop).toHaveCount(0);

  await test.step("opens when the image is clicked", async () => {
    await image.click();
    await expect(backdrop).toBeVisible();
  });

  await test.step("stays open when the image is clicked again", async () => {
    await image.click();
    await expect(backdrop).toBeVisible();
  });

  await test.step("stays open when overlay controls are used", async () => {
    await page.locator("#side-by-side").click();
    await expect(backdrop).toBeVisible();
    await expect(page.locator(".img-preview")).toHaveCount(2);
  });

  await test.step("closes when clicking outside the image", async () => {
    await closeOverlay(page);
    // Closing also resets the side-by-side comparison.
    await expect(page.locator(".img-preview")).toHaveCount(1);
  });
});

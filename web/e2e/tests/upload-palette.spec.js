import { expect, test } from "@playwright/test";

import {
  convert,
  downloadMappedImage,
  expectError,
  gotoApp,
  uploadImage,
} from "../helpers.js";

const VALID_PALETTE = "e2e/fixtures/palette.json";
const INVALID_PALETTE = "e2e/fixtures/invalid-palette.json";

test("uses an uploaded palette for conversion", async ({ page }) => {
  await gotoApp(page);
  await uploadImage(page);

  await page
    .getByTitle("Select palette to use from file.")
    .setInputFiles(VALID_PALETTE);

  const custom = page.locator(".uploaded-palette .option");
  await expect(custom).toBeVisible();
  await expect(custom.locator(".option-badge")).toHaveText("Custom");
  await expect(custom.locator(".option-select")).toHaveAttribute(
    "aria-pressed",
    "true",
  );

  await convert(page);
  expect(await downloadMappedImage(page)).toBe(
    "diagonal_rgb_gradient_500x500-mapped.png",
  );

  // Close the image overlay before interacting with the sidebar again.
  await page.locator(".img-preview-btn").click();

  await page.getByRole("button", { name: "click to remove" }).click();
  await expect(page.locator(".uploaded-palette")).toHaveCount(0);
  await expect(
    page.getByTitle("Select palette to use from file."),
  ).toBeVisible();
  await expect(page.locator('.option-select[aria-pressed="true"]')).toHaveCount(
    0,
  );
});

test("rejects an invalid uploaded palette and resets the selection", async ({ page }) => {
  await gotoApp(page);

  await page
    .getByTitle("Select palette to use from file.")
    .setInputFiles(INVALID_PALETTE);

  await expectError(page, "The uploaded palette is invalid.");
  await expect(page.locator(".uploaded-palette")).toHaveCount(0);
  await expect(
    page.getByTitle("Select palette to use from file."),
  ).toBeVisible();
  await expect(page.locator('.option-select[aria-pressed="true"]')).toHaveCount(
    0,
  );
});

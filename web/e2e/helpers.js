import { expect } from "@playwright/test";

/** Test image mounted into the e2e container at /app/test-imgs. */
export const TEST_IMAGE = "test-imgs/diagonal_rgb_gradient_500x500.png";

/** Navigate to the app and wait until the palette list has rendered. */
export async function gotoApp(page) {
  await page.goto("/");
  await expect(page.getByRole("list", { name: "Palette" })).toBeVisible();
}

/**
 * Upload an image (the standard test image by default). Waits for the
 * FileReader to finish, which is observable through the upload label switching
 * to the file name.
 */
export async function uploadImage(page, file = TEST_IMAGE) {
  const name = file.split("/").pop();

  await page.getByTitle("Select image to map.").setInputFiles(file);
  await expect(page.locator(".upload-btn")).toHaveText(name);
}

/** Filter the palette list, then select the first matching palette. */
export async function selectPalette(page, name) {
  await page.getByRole("searchbox", { name: "Search palette" }).fill(name);

  const row = page
    .getByRole("list", { name: "Palette" })
    .locator(".option")
    .first();

  await expect(row).toBeVisible();
  await row.locator(".option-select").click();

  return row;
}

/** Run the conversion and wait for the mapped image to be rendered. */
export async function convert(page) {
  await page.locator("#submit").click();
  await expect(page.locator(".img-preview").first()).toHaveAttribute(
    "src",
    /^blob:/,
  );
}

/** Open the image overlay and return the suggested download file name. */
export async function downloadMappedImage(page) {
  await page.locator(".img-preview-btn").click();

  const download_promise = page.waitForEvent("download");
  await page.locator("#download").click();

  return (await download_promise).suggestedFilename();
}

/** Click outside the image (on the dimmed background) to close the overlay. */
export async function closeOverlay(page) {
  await page.mouse.click(5, 5);
  await expect(page.locator(".backdrop")).toHaveCount(0);
}

/** Assert an error notification containing `text` is shown. */
export function expectError(page, text) {
  return expect(
    page.locator(".notification.ERROR .text").filter({ hasText: text }).first(),
  ).toBeVisible();
}

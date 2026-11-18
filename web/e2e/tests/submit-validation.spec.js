import { expect, test } from "@playwright/test";

import { expectError, gotoApp, uploadImage } from "../helpers.js";

test("requires an image before converting", async ({ page }) => {
  await gotoApp(page);

  const submit = page.locator("#submit");
  await submit.click();

  await expectError(page, "No image uploaded. Nothing to convert.");
  await expect(submit).toBeEnabled();
});

test("requires a palette before converting", async ({ page }) => {
  await gotoApp(page);
  await uploadImage(page);

  const submit = page.locator("#submit");
  await submit.click();

  await expectError(page, "No palette set. What colors should be used?");
  await expect(submit).toBeEnabled();
});

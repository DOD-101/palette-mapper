import { expect, test } from "@playwright/test";

import {
  convert,
  downloadMappedImage,
  gotoApp,
  selectPalette,
  uploadImage,
} from "../helpers.js";

test("convert image with pre-defined palette", async ({ page }, testInfo) => {
  await gotoApp(page);

  const algorithm = "ManhattanDistance";

  await test.step("select algorithm", async () => {
    const algorithm_select = page.getByRole("combobox", {
      name: "Algorithm",
    });

    await algorithm_select.selectOption(algorithm);
    await expect(algorithm_select).toHaveValue(algorithm);
  });

  const row = await test.step("select palette", () =>
    selectPalette(page, "tokyonightstorm"));

  await test.step("upload image", () => uploadImage(page));

  await test.step("convert", () => convert(page));

  await test.step("download the mapped image", async () => {
    const label = await row.locator(".option-label").innerText();
    const badge = await row
      .locator(".option-badge.toggle .toggle-option.active")
      .innerText();

    expect(await downloadMappedImage(page)).toBe(
      `diagonal_rgb_gradient_500x500-${badge}${label}.png`,
    );
  });

  const sc = await page.screenshot();
  await testInfo.attach("screenshot", {
    body: sc,
    contentType: "image/png",
  });
});

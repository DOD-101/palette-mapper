import { expect, test } from "@playwright/test";

test("convert image with pre-defined palette", async ({ page }, testInfo) => {
  await page.goto("/");

  const algorithm = "ManhattanDistance";

  const select_algorithm = test.step("select algorithm", () => {
    return page.locator("#algorithms").selectOption(algorithm);
  });
  await expect(await select_algorithm).toEqual([algorithm]);

  const palette = "base16::Everforest";

  const select_palette = test.step("select palette", () => {
    return page.locator("#base-palettes").selectOption(palette);
  });
  await expect(await select_palette).toEqual([palette]);

  await test.step("upload image", async () => {
    return await page
      .locator("#file_upload")
      .setInputFiles("test-imgs/diagonal_rgb_gradient_500x500.png");
  });

  await test.step("convert", async () => {
    return await page.locator("#submit").click();
  });

  const img_src = await test.step("check output", async () => {
    return await page.locator("#img_preview").getAttribute("src");
  });
  await expect(await img_src).toMatch(/blob:/);

  const sc = await page.screenshot();
  await testInfo.attach("screenshot", {
    body: sc,
    contentType: "image/png",
  });

  expect;
});

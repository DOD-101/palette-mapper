import { expect, test } from "@playwright/test";

test("convert image with pre-defined palette", async ({ page }, testInfo) => {
  await page.goto("/");

  const algorithm = "ManhattanDistance";

  await test.step("select algorithm", async () => {
    const algorithm_select = page.getByRole("combobox", {
      name: "Algorithm",
    });

    await algorithm_select.selectOption(algorithm);
    await expect(algorithm_select).toHaveValue(algorithm);
  });

  await test.step("select palette", () => {
    return page
      .getByRole("list", { name: "Palette" })
      .locator(".option-select")
      .nth(4)
      .click();
  });

  await test.step("upload image", async () => {
    return await page
      .getByTitle("Select image to map.")
      .setInputFiles("test-imgs/diagonal_rgb_gradient_500x500.png");
  });

  await test.step("convert", async () => {
    return await page.locator("#submit").click();
  });

  const img_src = await test.step("check output", async () => {
    return await page.locator(".img-preview");
  });
  await expect(await img_src).toHaveAttribute("src", /blob:/);

  const sc = await page.screenshot();
  await testInfo.attach("screenshot", {
    body: sc,
    contentType: "image/png",
  });

  expect;
});

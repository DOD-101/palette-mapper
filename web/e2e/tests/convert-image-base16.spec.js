import { expect, test } from "@playwright/test";

test("convert image with pre-defined palette", async ({ page }, testInfo) => {
  await page.goto("/");

  const algorithm = "ManhattanDistance";

  const algorithm_option = page.getByRole("option", { name: algorithm });

  await test.step("select algorithm", () => algorithm_option.click());
  await expect(algorithm_option).toHaveAttribute("aria-selected", "true");

  await test.step("select palette", () => {
    return page
      .getByRole("listbox", { name: "Palette" })
      .getByRole("option")
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

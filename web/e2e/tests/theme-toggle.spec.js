import { expect, test } from "@playwright/test";

import { gotoApp } from "../helpers.js";

test.describe("dark system preference", () => {
  test.use({ colorScheme: "dark" });

  test("starts dark, toggles to light and back", async ({ page }) => {
    await gotoApp(page);

    const html = page.locator("html");
    const toggle = page.getByRole("button", { name: "Change color theme" });

    await expect(html).toHaveAttribute("data-theme", "dark");

    await toggle.click();
    await expect(html).toHaveAttribute("data-theme", "light");

    await toggle.click();
    await expect(html).toHaveAttribute("data-theme", "dark");
  });
});

test.describe("light system preference", () => {
  test.use({ colorScheme: "light" });

  test("starts light and toggles to dark", async ({ page }) => {
    await gotoApp(page);

    const html = page.locator("html");
    const toggle = page.getByRole("button", { name: "Change color theme" });

    await expect(html).toHaveAttribute("data-theme", "light");

    await toggle.click();
    await expect(html).toHaveAttribute("data-theme", "dark");
  });
});

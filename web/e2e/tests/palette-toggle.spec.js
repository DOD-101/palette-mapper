import { expect, test } from "@playwright/test";

test("groups duplicate base16/24 themes behind a toggle", async ({ page }) => {
  await page.goto("/");

  const list = page.getByRole("listbox", { name: "Palette" });
  const options = list.getByRole("option");

  await expect(options.first()).toBeVisible();

  await page
    .getByRole("searchbox", { name: "Search palette" })
    .fill("tokyonightstorm");

  const row = options.first();
  const badge = row.locator(".option-badge");
  const active_variant = badge.locator(".toggle-option.active");

  await expect(options).toHaveCount(1);
  await expect(active_variant).toHaveText("Base16");
  await expect(row.locator(".swatches span")).toHaveCount(16);

  await badge.click();

  await expect(active_variant).toHaveText("Base24");
  await expect(row).toHaveAttribute("aria-selected", "true");
  await expect(row.locator(".swatches span")).toHaveCount(24);
});

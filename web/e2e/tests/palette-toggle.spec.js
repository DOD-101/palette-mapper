import { expect, test } from "@playwright/test";

test("groups duplicate base16/24 themes behind a toggle", async ({ page }) => {
  await page.goto("/");

  const list = page.getByRole("list", { name: "Palette" });
  const rows = list.locator(".option");

  await expect(rows.first()).toBeVisible();

  await page
    .getByRole("searchbox", { name: "Search palette" })
    .fill("tokyonightstorm");

  const row = rows.first();
  const badge = row.locator(".option-badge.toggle");
  const active_variant = badge.locator(".toggle-option.active");

  await expect(rows).toHaveCount(1);
  await expect(active_variant).toHaveText("Base16");
  await expect(row.locator(".swatches span")).toHaveCount(16);

  await badge.click();

  await expect(active_variant).toHaveText("Base24");
  await expect(row.locator(".option-select")).toHaveAttribute(
    "aria-pressed",
    "true",
  );
  await expect(row.locator(".swatches span")).toHaveCount(24);
});

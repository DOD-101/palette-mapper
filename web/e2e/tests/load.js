import { test, expect } from "@playwright/test";

test("static build works", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByText("Sign in")).toBeVisible();

    await page.getByRole("button", { name: "Sign in" }).click();

    await expect(page).toHaveURL("/dashboard");
});

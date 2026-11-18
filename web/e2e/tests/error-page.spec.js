import { expect, test } from "@playwright/test";

test("404 page keeps the site title above the message", async ({
  page,
}, testInfo) => {
  await page.goto("/this-page-does-not-exist");

  await expect(
    page.getByRole("heading", { name: "404 - Page Not Found" }),
  ).toBeVisible();

  const home = page.getByRole("link", { name: "Go back to the homepage" });
  await expect(home).toBeVisible();

  const title = await page.locator("h1").boundingBox();
  const message = await page.locator(".error-container").boundingBox();

  expect(title).not.toBeNull();
  expect(message).not.toBeNull();

  // The message block must start below the title and stay inside the viewport.
  expect(message.y).toBeGreaterThanOrEqual(title.y + title.height - 1);
  expect(message.y + message.height).toBeLessThanOrEqual(
    page.viewportSize().height + 20,
  );

  // The layout must not introduce horizontal scrolling.
  const scroll_width = await page.evaluate(
    () => document.documentElement.scrollWidth,
  );
  const client_width = await page.evaluate(
    () => document.documentElement.clientWidth,
  );
  expect(client_width).toBeLessThanOrEqual(scroll_width);

  const sc = await page.screenshot();
  await testInfo.attach("screenshot", {
    body: sc,
    contentType: "image/png",
  });

  await home.click();
  await expect(page).toHaveURL("/");
});

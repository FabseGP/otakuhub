import { test, expect } from "@playwright/test";

test("homepage has title and heading text", async ({ page }) => {
  await page.goto("http://localhost:9999/");

  await expect(page).toHaveTitle("Welcome to OtakuHub!");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
});

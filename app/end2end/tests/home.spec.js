import { test, expect } from "@playwright/test";

test("should redirect to login page", async ({ page }) => {
    await page.goto("http://localhost:8000/");

    await expect(page).toHaveTitle("Login | Mango³ ID (dev)");

    await expect(page.locator("h1")).toHaveText("Login");
});

import { test, expect } from "@playwright/test";

test("homepage has title and heading text", async ({ page }) => {
    await page.goto("http://localhost:8000/");

    await expect(page).toHaveTitle("Home | Mango³ ID");

    await expect(page.locator("h1")).toHaveText("Home");
});

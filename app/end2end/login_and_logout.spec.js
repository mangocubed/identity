import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker/locale/en";
import { loginAndGoToHome, waitForSplash } from "./shared_expects";

test("should redirect to login page when is not logged in", async ({ page }) => {
    await page.goto("/");

    await waitForSplash(page);

    await expect(page).toHaveURL("/login");
    await expect(page.locator("h1", { hasText: "Login" })).toBeVisible();
});

test("should login and logout a user", async ({ page }) => {
    const user = await loginAndGoToHome(page);

    await page.getByRole("button", { name: `@${user.username}` }).click();
    await page.locator("a", { hasText: "Logout" }).click();

    await expect(page.getByText("Are you sure you want to logout?")).toBeVisible();

    await page.getByRole("button", { name: "Accept" }).click();

    await expect(page.locator("h1", { hasText: "Login" })).toBeVisible();
});

test("should fail to login a user", async ({ page }) => {
    await page.goto("/login");

    await waitForSplash(page);

    await expect(page.locator("h1", { hasText: "Login" })).toBeVisible();

    await page.locator("#username_or_email").fill(faker.internet.username());
    await page.locator("#password").fill(faker.internet.password());

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("Failed to authenticate user")).toBeVisible();
});

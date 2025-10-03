import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker/locale/en";
import { waitForSplash } from "./shared_expects";

test("should register a new user", async ({ page }) => {
    const username = faker.internet.username().substring(0, 16);

    await page.goto("/register");

    await waitForSplash(page);

    await expect(page.locator("h1", { hasText: "Register" })).toBeVisible();

    await page.locator("#username").fill(username);
    await page.locator("#email").fill(faker.internet.email());
    await page.locator("#password").fill(faker.internet.password());
    await page.locator("#full_name").fill(faker.person.fullName());
    await page.locator("#birthdate").fill(faker.date.birthdate().toISOString().split("T")[0]);
    await page.locator("#country_alpha2").selectOption(faker.location.countryCode());

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("User created successfully")).toBeVisible();

    await page.getByRole("button", { name: "Ok" }).click();
});

test("should fail to register a new user", async ({ page }) => {
    await page.goto("/register");

    await waitForSplash(page);

    await expect(page.locator("h1", { hasText: "Register" })).toBeVisible();

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("Failed to create user")).toBeVisible();
});

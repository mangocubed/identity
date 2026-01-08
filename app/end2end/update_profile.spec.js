import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker/locale/en";
import { loginAndGoToHome, waitForSplash } from "./shared_expects";

test("should redirect to login page when is not logged in", async ({ page }) => {
    await page.goto("/edit-profile");

    await waitForSplash(page);

    await expect(page).toHaveURL("/login");
    await expect(page.locator("h1", { hasText: "Login" })).toBeVisible();
});

test("should be a link to edit user profile", async ({ page }) => {
    const _user = await loginAndGoToHome(page);

    await waitForSplash(page);

    page.getByRole("link", { name: "Edit profile" }).click();

    await expect(page).toHaveURL("/edit-profile");
    await expect(page.locator("h1", { hasText: "Edit Profile" })).toBeVisible();
});

test("should update profile", async ({ page }) => {
    const _user = await loginAndGoToHome(page);

    await page.goto("/edit-profile");

    await waitForSplash(page);

    await expect(page.locator("h1", { hasText: "Edit Profile" })).toBeVisible();

    await page.locator("#full_name").fill(faker.person.fullName());
    await page.locator("#birthdate").fill(faker.date.birthdate().toISOString().split("T")[0]);
    await page.locator("#country_alpha2").selectOption(faker.location.countryCode());

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("Profile updated successfully")).toBeVisible();

    await page.getByRole("button", { name: "Ok" }).click();

    await expect(page).toHaveURL("/");
    await expect(page.locator("h1", { hasText: "Home" })).toBeVisible();
});

test("should fail to update profile when form is empty", async ({ page }) => {
    const _user = await loginAndGoToHome(page);

    await page.goto("/edit-profile");

    await waitForSplash(page);

    await expect(page.locator("h1", { hasText: "Edit Profile" })).toBeVisible();

    await page.locator("#full_name").clear();
    await page.locator("#birthdate").clear();

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("Failed to update profile")).toBeVisible();
});

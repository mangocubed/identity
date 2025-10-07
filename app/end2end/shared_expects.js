import { expect } from "@playwright/test";

export async function waitForSplash(page) {
    await expect(page.locator(".splash")).toHaveClass(/splash-hidden/);
}

export async function loginAndGoToHome(page) {
    const username = faker.internet.username().substring(0, 16);
    const email = `${username}@example.com`;
    const password = faker.internet.password();
    const fullName = faker.person.fullName();
    const birthdate = faker.date.birthdate().toISOString().split("T")[0];
    const country = faker.location.countryCode();
    const result = execSync(
        `cargo run --package identity-cli --release create-user \
                --username '${username}' --email '${email}' --password '${password}' \
                --full-name '${fullName}' --birthdate '${birthdate}' --country '${country}'`,
    );

    expect(result.toString()).toContain("User created successfully.");

    await page.goto("/login");

    await waitForLoadingOverlay(page);

    await expect(page.locator("h1", { hasText: "Login" })).toBeVisible();

    await page.locator("#username_or_email").fill(username);
    await page.locator("#password").fill(password);

    await page.getByRole("button", { name: "Submit" }).click();

    await expect(page.getByText("User authenticated successfully")).toBeVisible();

    await page.getByRole("button", { name: "Ok" }).click();

    await expect(page).toHaveURL("/");
    await expect(page.locator("h1", { hasText: "Home" })).toBeVisible();

    return {
        username,
    };
}

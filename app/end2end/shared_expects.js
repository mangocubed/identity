import { expect } from "@playwright/test";

export async function waitForSplash(page) {
    await expect(page.locator(".splash")).toHaveClass(/splash-hidden/);
}

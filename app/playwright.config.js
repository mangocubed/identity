import { devices, defineConfig } from "@playwright/test";

export default defineConfig({
    testDir: "./end2end",

    outputDir: "./end2end/output/",

    timeout: 60000,

    expect: {
        timeout: 10000,
    },

    fullyParallel: true,

    forbidOnly: !!process.env.CI,

    retries: process.env.CI ? 2 : 0,

    workers: process.env.CI ? 1 : undefined,

    reporter: process.env.CI ? "dot" : "list",

    use: {
        actionTimeout: 0,
        trace: "on-first-retry",
        headless: !!process.env.CI || process.env.HEADLESS != "false",
    },

    projects: [
        {
            name: "chromium",
            use: {
                ...devices["Desktop Chrome"],
            },
        },
        {
            name: "firefox",
            use: {
                ...devices["Desktop Firefox"],
            },
        },
    ],

    webServer: {
        cwd: "..",
        command: `npm run build && cargo build --bin identity-cli --release && \
            dx build --package identity-app --release --web && \
            dx serve --package identity-app --release --web`,
        port: 8080,
        timeout: 3600000,
        reuseExistingServer: !process.env.CI,
        stdout: "pipe",
    },
});

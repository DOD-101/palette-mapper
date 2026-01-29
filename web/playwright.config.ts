import { defineConfig } from "@playwright/test";

export default defineConfig({
    testDir: "./e2e/tests",
    timeout: 30_000,
    retries: 0,

    use: {
        baseURL: "http://localhost:8099",
        headless: true,
    },

    webServer: [
        {
            command: "deno task dev --port 8099",
            port: 8099,
        },
    ],
});

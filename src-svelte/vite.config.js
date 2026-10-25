import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import process from "node:process";

import { BuildEnv } from "./env/build.env.ts";
import { PublicEnv } from "./env/public.env.ts";
import { envCheck } from "./env/vite.ts";


// Partial, because svelte-check and the editor load this file without the env.
// The `envCheck` plugin reports any missing or invalid vars once Vite
// actually starts
const env = BuildEnv.loadPartial(process.env);

// Dev server settings come from .env.dev, which the root `dev:*` tasks load
const AXUM_URL = env.AXUM_TARGET_URL;
const BROWSER_HOST = env.VITE_BROWSER_HOST;
const BROWSER_PORT = env.VITE_BROWSER_PORT;

// Set by the Tauri CLI when it runs Vite for the desktop app
const IS_TAURI = env.TAURI_ENV_PLATFORM !== undefined;
const TAURI_HOST = env.TAURI_DEV_HOST;
const TAURI_PORT = env.VITE_TAURI_PORT;


// https://vite.dev/config/
// Vite options tailored for Tauri development are only applied during
// `tauri dev` or `tauri build`
export default defineConfig(() => ({
  plugins: [sveltekit(), envCheck()],


  // Env vars that browser code can read through `import.meta.env`
  envPrefix: PublicEnv.keys,


  // Prevent Vite from obscuring rust errors
  clearScreen: IS_TAURI ? false : undefined,


  server: {
    host: IS_TAURI
      ? TAURI_HOST ?? "127.0.0.1"
      : BROWSER_HOST,

    port: IS_TAURI
      ? TAURI_PORT
      : BROWSER_PORT,

    strictPort: true,

    // The browser loads the public env schema from `env/`, which is
    // outside `src`.
    // Only the files it imports are listed, so the build-only ones
    // aren't served
    fs: {
      allow: [
        "env/public.env.ts",
        "env/index.ts",
        "env/zod.ts",
      ],
    },

    hmr: IS_TAURI && TAURI_HOST && TAURI_PORT
      ? {
          protocol: "ws",
          host: TAURI_HOST,
          port: TAURI_PORT+1,
        }
      : undefined,

    // The browser calls the API on its own origin, so forward /api to the Axum server
    proxy: AXUM_URL
      ? { "/api": AXUM_URL }
      : undefined,
  },
}));

import { defineEnv } from "./index.ts";
import { PublicEnv } from "./public.env.ts";
import * as z from "./zod.ts";


/** A TCP port: a whole number from 1 to 65535 */
const port = z.coerce.number().check(z.int(), z.minimum(1), z.maximum(65535));


/** Every env var `vite.config.js` reads, through `process.env` */
export const BuildEnv = defineEnv({
  /** Interface the browser's Vite dev server listens on */
  VITE_BROWSER_HOST: z.string(),

  /** Vite dev server port for the browser */
  VITE_BROWSER_PORT: port,

  /** Vite dev server port for the Tauri WebView */
  VITE_TAURI_PORT: port,

  /** Where the dev server proxies `/api`, with the same rule as in `PublicEnv` */
  AXUM_TARGET_URL: PublicEnv.shape.AXUM_TARGET_URL,

  /** Set by the Tauri CLI when it runs Vite */
  TAURI_ENV_PLATFORM: z.optional(z.string()),

  /** Set by the Tauri CLI when the app runs on a mobile device */
  TAURI_DEV_HOST: z.optional(z.string()),
});

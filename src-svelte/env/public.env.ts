import { defineEnv } from "./index.ts";
import * as z from "./zod.ts";


/**
 * Every env var the app's own code reads, through `import.meta.env`
 *
 * These are built into the app's JavaScript, where anyone can read them,
 * so never put a secret here.
 * 
 * Only vars named here can reach the app: `vite.config.js`
 * sets Vite's `envPrefix` from these keys.
 */
export const PublicEnv = defineEnv({
  /** Where clients send API requests */
  AXUM_TARGET_URL: z.url(),
});

import process from "node:process";
import type { Plugin } from "vite";

import { BuildEnv } from "./build.env.ts";
import { PublicEnv } from "./public.env.ts";


/**
 * Checks the env once Vite actually starts, so a bad var stops
 * the dev server or build.
 *
 * svelte-check and the editor also load `vite.config.js`, without the env, 
 * but they never run these hooks.
 */
export function envCheck(): Plugin {
  return {
    name: "env-check",

    // Runs for every build, and when the dev server starts
    buildStart() {
      PublicEnv.load(process.env);
    },

    // Runs only when the dev server starts
    // builds don't use the dev server's vars
    configureServer() {
      BuildEnv.load(process.env);
    },
  };
}

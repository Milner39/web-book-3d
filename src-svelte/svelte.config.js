// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { sveltePreprocess } from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: sveltePreprocess(),

  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),

    // The env schemas live in `env/`, outside `src`
    alias: {
      "$public-env": "env/public.env.ts",
    },

    // Type-check `env/` along with `src`
    typescript: {
      config: (config) => {
        config.include.push("../env/**/*.ts");
      },
    },
  },

  compilerOptions: {
    runes: true
  },
};

export default config;

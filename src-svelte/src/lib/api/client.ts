import { isTauri } from "$lib/platform";
import { PublicEnv } from "$public-env";

const env = PublicEnv.load(import.meta.env);

// Browser: same origin as the page:
// - Dev:  The Vite proxy
// - Prod: Axum serves the site, so just `/api` works.
//
// Tauri: Absolute URL
const base = isTauri() ? env.AXUM_TARGET_URL : "";


export class ApiError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

/** Calls the Axum API at `/api${path}` and parses the JSON response. */
export async function api<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${base}/api${path}`, init);

  if (!response.ok) {
    throw new ApiError(response.status, await response.text());
  }

  return response.json();
}

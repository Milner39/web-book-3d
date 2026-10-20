// Zod mini, with English error messages
//
// zod/mini ships without messages to stay small, so every error would just say
// "Invalid input". Schemas import `z` from here so the messages are always on.
// `en` is imported by name: going through `z.locales` would bundle 
// every language.

import { config } from "zod/mini";
import { en } from "zod/locales";

config(en());

export * from "zod/mini";

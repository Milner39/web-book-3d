// Env vars, checked against a schema
//
// A schema maps each var's name to the rule its value must pass. Rules are any
// Standard Schema validator.
// The schemas here use Zod mini, from `./zod`.
// Loading checks every var before failing, so every issue is reported.
// Values are trimmed, and a var set to an empty string counts as unset.

import type { StandardSchemaV1 } from "@standard-schema/spec";



/** The rule one env var's value must pass */
export type Rule = StandardSchemaV1;

/** Where vars are read from, e.g. `process.env` or `import.meta.env` */
export type Source = Record<string, unknown>;

/** A schema's checked values, each typed by its var's rule */
export type Values<Shape extends Record<string, Rule>> = {
  [Name in keyof Shape]: StandardSchemaV1.InferOutput<Shape[Name]>;
};


/** Declares a schema: each key is an env var's name, and each value is its rule */
export function defineEnv<Shape extends Record<string, Rule>>(shape: Shape) {
  const keys = Object.keys(shape);

  /** Checks every var, keeping the valid values and describing the rest */
  function check(source: Source) {
    const values: Record<string, unknown> = {};
    const issues: string[] = [];

    for (const name of keys) {
      const result = checkVar(name, shape[name], source[name]);
      if ("issue" in result) {
        issues.push(result.issue);
      } else {
        values[name] = result.value;
      }
    }
    return { values, issues };
  }

  return {
    /** Each var's rule, e.g. to reuse one in another schema */
    shape,

    /** Every var's name */
    keys,

    /** Returns every value, or throws an `EnvError` listing every problem */
    load(source: Source): Values<Shape> {
      const { values, issues } = check(source);
      if (issues.length > 0) {
        throw new EnvError(issues);
      }
      return values as Values<Shape>;
    },

    /** Returns only the valid values, and never throws */
    loadPartial(source: Source): Partial<Values<Shape>> {
      return check(source).values as Partial<Values<Shape>>;
    },
  };
}


/** Checks one var, describing the problem if it fails */
function checkVar(name: string, rule: Rule, raw: unknown): { value: unknown } | { issue: string } {
  const value = typeof raw === "string" ? (raw.trim() || undefined) : raw;

  const result = rule["~standard"].validate(value);
  if (result instanceof Promise) {
    throw new TypeError(`${name}: env rules must check synchronously`);
  }
  if (!result.issues) {
    return { value: result.value };
  }

  if (raw === undefined) {
    return { issue: `${name}: not set` };
  }
  if (value === undefined) {
    return { issue: `${name}: set but empty` };
  }
  // A rule's messages must not repeat the value, so secrets stay out of logs
  const messages = result.issues.map((issue) => issue.message);
  return { issue: `${name}: ${messages.join("; ")}` };
}


/** Every problem found while loading a schema */
export class EnvError extends Error {
  constructor(issues: string[]) {
    const plural = issues.length === 1 ? "" : "s";
    const heading = `invalid environment (${issues.length} problem${plural}):`;
    const list = issues.map((issue) => `\n  ${issue}`).join("");

    super(heading + list);
    this.name = "EnvError";
  }
}

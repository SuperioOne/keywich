import {z} from "zod";

const CharsetOptionSchema = z.object({
  charset: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
});

/**
 * Parses the provided options using the CharsetOptionSchema.
 * @param options The options to parse.
 * @return Returns undefined if parsing is successful,
 * otherwise returns an object with success set to false and a list of errors.
 * @type {import('./types.js').parse_CharsetOptions}
 */
export function parse_CharsetOptions(options) {
  const parseResult = CharsetOptionSchema.safeParse(options);

  if (parseResult.success) return undefined;

  return {
    success: false,
    error: {
      ...parseResult.error.flatten().fieldErrors
    }
  }
}
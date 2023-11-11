import {z} from "zod";

const KeyOptionsSchema = z.object({
  target_size: z.number().min(1).max(64),
  revision: z.number().min(0),
  charset: z.string().min(1),
  domain: z.string().min(1),
  user_name: z.string().min(1),
  notes: z.string().optional(),
  custom_icon: z.instanceof(File).optional(),
  tags: z.array(z.string().min(1)).optional(),
});

/**
 * Parses the provided options using the KeyOptionsSchema.
 * @param options The options to parse.
 * @return Returns undefined if parsing is successful,
 * otherwise returns an object with success set to false and a list of errors.
 * @type {import('./types.js').parse_KeyOptions}
 */
export function parse_KeyOptions(options) {
  const parseResult = KeyOptionsSchema.safeParse(options);

  if (parseResult.success) return undefined;

  return {
    success: false,
    error: {
      ...parseResult.error.flatten().fieldErrors
    }
  }
}
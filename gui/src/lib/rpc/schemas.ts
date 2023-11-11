import {z} from "zod";

export const KeyOptionsSchema = z.object({
  target_size: z.number().min(1).max(64),
  revision: z.number().min(0),
  charset: z.string().min(1),
  domain: z.string().min(1),
  user_name: z.string().min(1),
  notes: z.string().optional(),
  custom_icon: z.instanceof(File).optional(),
  tags: z.array(z.string().min(1)).optional(),
});

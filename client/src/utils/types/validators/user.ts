import { z } from "zod/v4";

export const userSchema = z.object({
  id: z.string(),
  email: z.email(),
  name: z.string().nullable().optional(),
  avatar_url: z.string().nullable().optional(),
  credits: z.number(),
  created_at: z.string(),
});

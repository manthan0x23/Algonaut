import { z } from "zod/v4";

export const codeLanguageSchema = z.enum(["cpp", "java", "python3"]);
export const roomSchema = z.object({
  id: z.string().min(1),
  alias: z.string(),
  objective: z.string(),
  capacity: z.number(),
  editors_scope_type: z.enum(["Open", "Strict"]),
  viewers_scope_type: z.enum(["Open", "Strict"]),
  allowed_editors: z.array(z.string()),
  allowed_viewers: z.array(z.string()),
  code: z.string().nullable(),
  code_language: codeLanguageSchema,
  created_by: z.string(),
  created_at: z.string(),
});

export const roomRoleSchema = z.enum(["creator", "editor", "viewer"]);

import type { z } from "zod/v4";
import type { codeLanguageSchema, roomSchema } from "./validators/room";

export type Room = z.infer<typeof roomSchema>;
export type CodeLanguage = z.infer<typeof codeLanguageSchema>;

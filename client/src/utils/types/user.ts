import type { z } from "zod/v4";
import type { userSchema } from "./validators/user";

export type User = z.infer<typeof userSchema>;

import { z } from "zod/v4";
import type { LucideIcon } from "lucide-react";

export const SideBarParamEnum = z.enum([
  "space",
  "members",
  "chat",
  "executions",
  "tarsy",
  "settings",
]);

export type SideBarParam = z.infer<typeof SideBarParamEnum>;

export interface SideBarOption {
  title: string;
  param: string;
  icon: LucideIcon;
}

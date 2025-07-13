import { z } from "zod/v4";
import { ChatMessageType } from "./types";

export const OutgoingChat = z.object({
  message_type: ChatMessageType,
  url: z.url().optional(),
  text: z.string().optional(),
  _blob: z.union([z.instanceof(Uint8Array), z.array(z.number())]).optional(),
  timestamp: z.string().refine((s) => !isNaN(Date.parse(s)), {
    message: "Invalid timestamp",
  }),
});

export type OutgoingChat = z.infer<typeof OutgoingChat>;

export const OutgoingMessage = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("chat"),
    ...OutgoingChat.shape,
  }),
]);

export type OutgoingMessage = z.infer<typeof OutgoingMessage>;

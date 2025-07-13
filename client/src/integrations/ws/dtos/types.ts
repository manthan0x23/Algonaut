import { z } from "zod/v4";

export const ChatMessageType = z.enum([
  "text",
  "video",
  "audio",
  "file",
  "image",
]);

export type ChatMessageType = z.infer<typeof ChatMessageType>;

export const ChatType = z.enum(["announcement", "message"]);
export type ChatType = z.infer<typeof ChatType>;

export const MessageIden = z.enum([
  "broadcast",
  "chat",
  "execution",
  "room_members",
  "error",
]);

export type MessageIden = z.infer<typeof MessageIden>;

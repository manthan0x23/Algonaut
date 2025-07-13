import { z } from "zod/v4";
import { ChatMessageType, ChatType } from "./types";

export const RoomMemberType = z.enum(["creator", "editor", "viewer"]);

export type RoomMemberType = z.infer<typeof RoomMemberType>;

export const RoomMember = z.object({
  uid: z.string(),
  name: z.string().nullable(),
  email: z.email(),
  role: RoomMemberType,
});

export type RoomMember = z.infer<typeof RoomMember>;

export const IncomingRoomMembers = z.object({
  type: z.literal("room_members"),
  iden: z.literal("room_members"),
  members: z.array(RoomMember),
});
export type IncomingRoomMembers = z.infer<typeof IncomingRoomMembers>;

export const IncomingBroadcast = z.object({
  type: z.literal("broadcast"),
  iden: z.literal("broadcast"),
  message: z.string(),
});
export type IncomingBroadcast = z.infer<typeof IncomingBroadcast>;

export const IncomingChat = z.object({
  iden: z.literal("chat"),
  type: ChatType,
  message: z.string().nullable(),
  from: z.string(),
  message_type: ChatMessageType,
  url: z.url().nullable(),
  text: z.string().nullable(),
  blob: z.any().nullable(),
  timestamp: z.string().refine((s) => !isNaN(Date.parse(s)), {
    message: "Invalid timestamp",
  }),
});
export type IncomingChat = z.infer<typeof IncomingChat>;

export const IncomingError = z.object({
  type: z.literal("error"),
  iden: z.literal("error"),
  message: z.string(),
  errors: z.array(z.string()),
});
export type IncomingError = z.infer<typeof IncomingError>;

// @TODO
export const IncomingMessage = z.discriminatedUnion("type", [
  IncomingBroadcast,
  IncomingChat,
  IncomingRoomMembers,
  IncomingError,
]);
export type IncomingMessage = z.infer<typeof IncomingMessage>;

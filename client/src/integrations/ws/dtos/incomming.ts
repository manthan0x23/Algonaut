import { z } from "zod/v4";

// --------------------
// Shared enums
// --------------------
export const ChatType = z.enum(["message", "announcement"]);
export type ChatType = z.infer<typeof ChatType>;

export const ChatMessageType = z.enum([
  "text",
  "video",
  "audio",
  "file",
  "image",
]);
export type ChatMessageType = z.infer<typeof ChatMessageType>;

export const RoomMemberType = z.enum(["creator", "editor", "viewer"]);
export type RoomMemberType = z.infer<typeof RoomMemberType>;

// --------------------
// UserMinimal
// --------------------
export const UserMinimal = z.object({
  id: z.string(),
  name: z.string().nullable().optional(),
  email: z.string().nullable().optional(),
  avatar_url: z.string().nullable().optional(),
});
export type UserMinimal = z.infer<typeof UserMinimal>;

// --------------------
// ChatModel (flattened)
// --------------------
export const ChatModel = z.object({
  id: z.string(),
  type: z.string(),
  text: z.string().nullable().optional(),
  file: z.string().nullable().optional(),
  room_id: z.string(),
  user_id: z.string(),
  created_at: z.string().refine((s) => !isNaN(Date.parse(s)), {
    message: "Invalid created_at",
  }),
});
export type ChatModel = z.infer<typeof ChatModel>;

// --------------------
// RoomMember
// --------------------
export const RoomMember = z.object({
  uid: z.string(),
  name: z.string().nullable(),
  email: z.string().email(),
  role: RoomMemberType,
});
export type RoomMember = z.infer<typeof RoomMember>;

// --------------------
// IncomingRoomMembers
// --------------------
export const IncomingRoomMembers = z.object({
  iden: z.literal("room_members"),
  members: z.array(RoomMember),
});
export type IncomingRoomMembers = z.infer<typeof IncomingRoomMembers>;

// --------------------
// IncomingBroadcast
// --------------------
export const IncomingBroadcast = z.object({
  iden: z.literal("broadcast"),
  message: z.string(),
});
export type IncomingBroadcast = z.infer<typeof IncomingBroadcast>;

// --------------------
// IncomingBroadcast
// --------------------
export const IncommingCrdtUpdate = z.object({
  iden: z.literal("crdt_update"),
  update: z.array(z.array(z.number())),
});
export type IncommingCrdtUpdate = z.infer<typeof IncommingCrdtUpdate>;

// --------------------
// IncomingChat (matching OutgoingChat)
// --------------------
export const IncomingChat = z.object({
  iden: z.literal("chat"),
  ...ChatModel.shape,
  sender: UserMinimal,
});
export type IncomingChat = z.infer<typeof IncomingChat>;

// --------------------
// IncomingError
// --------------------
export const IncomingError = z.object({
  iden: z.literal("error"),
  message: z.string(),
  errors: z.array(z.string()),
});
export type IncomingError = z.infer<typeof IncomingError>;

// --------------------
// Incoming union
// --------------------
export const IncomingMessage = z.union([
  IncomingBroadcast,
  IncomingChat,
  IncomingRoomMembers,
  IncomingError,
  IncommingCrdtUpdate,
  z.literal("ping"),
  z.literal("pong"),
]);

export type IncomingMessage = z.infer<typeof IncomingMessage>;

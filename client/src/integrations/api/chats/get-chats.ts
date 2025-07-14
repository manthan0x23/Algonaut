import { Env } from "@/lib/env";
import { getAxiosResponseError } from "@/lib/error";
import axios from "axios";
import { z } from "zod/v4";

export const getRoomChatsQueryParamsSchema = z.object({
  page: z.number().default(1).optional(),
  room_id: z.string(),
});

export type GetRoomChatsQueryParams = z.infer<
  typeof getRoomChatsQueryParamsSchema
>;

export type GetRoomChatsResponseT = z.infer<typeof getChatsResponseSchema>;

export const ChatSchema = z.object({
  id: z.string(),
  room_id: z.string(),
  user_id: z.string(),
  text: z.string().nullable(),
  file: z.string().nullable(),
  type: z.enum(["text", "image", "file", "video", "audio"]),
  created_at: z.string().nullable(),
  sender: z.object({
    id: z.string(),
    name: z.string().nullable(),
    email: z.string().nullable(),
    avatar_url: z.url().nullable(),
  }),
});

export type ChatSchema = z.infer<typeof ChatSchema>;

export const getChatsResponseSchema = z.object({
  message: z.string(),
  data: z.object({
    chats: z.array(ChatSchema),
    total_items: z.number(),
    total_pages: z.number(),
  }),
});

export const GetRoomChats = async (
  queryParams: GetRoomChatsQueryParams
): Promise<GetRoomChatsResponseT> => {
  try {
    const parsed = getRoomChatsQueryParamsSchema.parse(queryParams);

    const res = await axios.get(`${Env.server_url}/api/rooms/chats`, {
      params: parsed,
      withCredentials: true,
    });

    console.log(res.data);

    const parse = getChatsResponseSchema.safeParse(res.data);

    if (!parse.success) {
      console.log(parse.error.issues);
      throw new Error("Invalid response format");
    }

    return parse.data;
  } catch (err) {
    const parsedError = getAxiosResponseError(err as Error);
    console.error("API Error:", parsedError || err);
    throw err;
  }
};

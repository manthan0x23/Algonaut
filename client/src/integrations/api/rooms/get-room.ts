import { Env } from "@/lib/env";
import { getAxiosResponseError } from "@/lib/error";
import { roomRoleSchema, roomSchema } from "@/utils/types/validators/room";
import axios from "axios";
import { z } from "zod/v4";

export const getUserRoomsQueryParamsSchema = z.object({
  page: z.number().optional(),
  only_created: z.boolean().optional(),
  only_joined: z.boolean().optional(),
});

export type GetUserRoomsQueryParams = z.infer<
  typeof getUserRoomsQueryParamsSchema
>;

export type GetUserRoomsResponseT = z.infer<typeof getRoomsResponseSchema>;

export const getRoomsResponseSchema = z.object({
  message: z.string(),
  data: z.object({
    rooms: z.array(
      z.object({
        room: roomSchema,
        role: roomRoleSchema,
      })
    ),
    total_items: z.number(),
    total_pages: z.number(),
  }),
});

export const GetUserRooms = async (
  queryParams: GetUserRoomsQueryParams
): Promise<GetUserRoomsResponseT> => {
  try {
    const parsed = getUserRoomsQueryParamsSchema.parse(queryParams);

    const res = await axios.get(`${Env.server_url}/api/rooms/user`, {
      params: parsed,
      withCredentials: true,
    });

    return getRoomsResponseSchema.parse(res.data);
  } catch (err) {
    const parsedError = getAxiosResponseError(err as Error);
    console.error("API Error:", parsedError || err);
    throw err;
  }
};

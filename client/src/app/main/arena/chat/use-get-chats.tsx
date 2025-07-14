import {
  GetRoomChats,
  type GetRoomChatsResponseT,
} from "@/integrations/api/chats/get-chats";
import { useChatsStore } from "@/integrations/ws/store/chats";
import type { ApiError } from "@/lib/error";
import { useQuery } from "@tanstack/react-query";
import { useParams } from "@tanstack/react-router";
import { toast } from "sonner";
import { useEffect } from "react";

export const useGetChats = () => {
  const { roomId } = useParams({
    from: "/arena/$roomId",
  });

  const chatStore = useChatsStore();

  const query = useQuery<GetRoomChatsResponseT, ApiError>({
    queryKey: ["api", "rooms", "chat", roomId, 1],
    queryFn: () => GetRoomChats({ room_id: roomId, page: 1 }),
    retry: false,
    enabled: !!roomId, // <- avoid calling if no roomId yet
  });

  useEffect(() => {
    if (query.error?.response?.data.message) {
      toast.error(query.error.response.data.message);
    }
  }, [query.error]);

  useEffect(() => {
    if (query.isSuccess && query.data) {
      const { chats, total_items, total_pages } = query.data.data;

      const isSame =
        chatStore.chats === chats &&
        chatStore.totalItems === total_items &&
        chatStore.totalPages === total_pages;

      if (!isSame) {
        chatStore.setState(chats.reverse(), total_items, total_pages);
      }
    }
  }, [query.isSuccess, query.data, chatStore]);

  return query;
};

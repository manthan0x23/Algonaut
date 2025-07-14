import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import type { ChatSchema } from "@/integrations/api/chats/get-chats";

export interface ChatsState {
  chats: ChatSchema[];
  totalItems: number;
  totalPages: number;
  isLoading: boolean;
  error: string | null;

  pushFront: (item: ChatSchema) => void;
  pushBack: (item: ChatSchema) => void;
  setState: (
    items: ChatSchema[],
    totalItems: number,
    totalPages: number
  ) => void;
  clearChats: () => void;
}

export const useChatsStore = create<ChatsState>()(
  devtools(
    persist(
      (set) => ({
        chats: [],
        totalItems: 0,
        totalPages: 0,
        isLoading: false,
        error: null,

        pushFront: (item) =>
          set((state) => ({
            chats: [item, ...state.chats],
            totalItems: state.totalItems + 1,
          })),

        pushBack: (item) =>
          set((state) => ({
            chats: [...state.chats, item],
            totalItems: state.totalItems + 1,
          })),

        setState: (items, totalItems, totalPages) =>
          set({
            chats: items,
            totalItems,
            totalPages,
          }),

        clearChats: () =>
          set({
            chats: [],
            totalItems: 0,
            totalPages: 0,
            error: null,
          }),
      }),
      {
        name: "chats-store",
      }
    )
  )
);

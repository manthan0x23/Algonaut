"use client";

import { useChatsStore } from "@/integrations/ws/store/chats";
import { useAuthStore } from "@/store/auth-store";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Send } from "lucide-react";
import { cn } from "@/lib/utils";
import { useEffect, useRef } from "react";

export const ChatSection = () => {
  const { session } = useAuthStore();
  const { chats } = useChatsStore();

  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    scrollRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [chats]);

  if (!session) return null;

  const formatTime = (dateStr: string | Date) => {
    const date = new Date(dateStr);
    const now = Date.now();
    const diffMins = Math.floor((now - date.getTime()) / 60000);

    if (diffMins < 1) return "Just now";
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h ago`;
    return date.toLocaleDateString();
  };

  const isNewGroup = (idx: number) => {
    if (idx === 0) return true;
    const curr = chats[idx];
    const prev = chats[idx - 1];
    if (!prev) return true;

    const sameSender = curr.sender.id === prev.sender.id;
    const delta =
      Math.abs(
        new Date(curr.created_at!).getTime() -
          new Date(prev.created_at!).getTime()
      ) / 60000; // minutes

    return !(sameSender && delta < 10);
  };

  return (
    <div className="flex flex-col h-full w-full">
      <ScrollArea className="flex-1 p-4 space-y-2">
        {chats.map((chat, idx) => {
          if (chat.type !== "text") return null;

          const isMe = chat.sender.id === session.uid;
          const firstInGroup = isNewGroup(idx);

          return (
            <div
              key={chat.id}
              className={cn(
                "flex items-start gap-3",
                isMe && "flex-row-reverse"
              )}
            >
              {firstInGroup && (
                <Avatar className="h-8 w-8">
                  <AvatarImage
                    src={chat.sender.avatar_url || "/placeholder.svg"}
                  />
                  <AvatarFallback>
                    {chat.sender.name?.[0] ?? "U"}
                  </AvatarFallback>
                </Avatar>
              )}

              <div
                className={cn(
                  "flex flex-col",
                  isMe && "items-end",
                  !firstInGroup && "ml-11"
                )}
              >
                {firstInGroup && (
                  <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    {!isMe && (
                      <span className="font-medium text-foreground">
                        {chat.sender.name}
                      </span>
                    )}
                    <span>{formatTime(chat.created_at!)}</span>
                  </div>
                )}

                <div
                  className={cn(
                    "mt-1 px-3 py-2 rounded-lg max-w-xs text-sm",
                    isMe ? "bg-primary text-primary-foreground" : "bg-muted"
                  )}
                >
                  {chat.text}
                </div>
              </div>
            </div>
          );
        })}

        {/* Dummy div to scroll into */}
        <div ref={scrollRef} />
      </ScrollArea>

      <div className="p-4 border-t flex items-center gap-2">
        <Input placeholder="Type a message..." className="flex-1" />
        <Button size="icon">
          <Send className="w-4 h-4" />
        </Button>
      </div>
    </div>
  );
};

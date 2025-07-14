import {
  FileText,
  Users,
  MessageCircle,
  History,
  Bot,
  Settings,
} from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  TooltipProvider,
} from "@/components/ui/tooltip";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Link, useParams, useRouter, useSearch } from "@tanstack/react-router";
import { cn } from "@/lib/utils";
import { SideBarParamEnum, type SideBarOption } from "./dto";
import { useEffect } from "react";
import { toast } from "sonner";
import { ChatSection } from "../chat";
import { z } from "zod/v4";

export const ChatSchema = z.object({
  id: z.string(),
  room_id: z.string(),
  user_id: z.string(),
  message: z.string(),
  message_type: z.enum(["text", "image", "file", "video", "audio"]),
  created_at: z.string(),
  updated_at: z.string(),

  sender: z.object({
    uid: z.string(),
    name: z.string().optional(),
    email: z.string(),
  }),
});

const options: SideBarOption[] = [
  { title: "Space", param: "space", icon: FileText },
  { title: "Members", param: "members", icon: Users },
  { title: "Chat", param: "chat", icon: MessageCircle },
  { title: "Executions", param: "executions", icon: History },
  { title: "T.A.R.S.Y.", param: "tarsy", icon: Bot },
  { title: "Settings", param: "settings", icon: Settings },
];

export const ArenaPanel = () => {
  const { roomId } = useParams({
    from: "/arena/$roomId",
  });
  const router = useRouter();
  const params = useSearch({
    from: "/arena/$roomId",
  }) as { tab: SideBarOption["param"][number] };

  useEffect(() => {
    const check = SideBarParamEnum.safeParse(params.tab);

    if (check.error) {
      toast.error("Navigating to default tab");

      router.navigate({
        to: `/arena/${roomId}`,
        search: {
          tab: "space",
        },
        resetScroll: true,
        replace: true,
      });
    }
  }, [params]);

  return (
    <div className="h-full flex">
      {/* Main Content Area */}
      <div className="flex-1 bg-background border-r">
        <div className="h-12 border-b bg-muted/30 flex items-center px-4">
          <span className="text-sm font-medium text-muted-foreground capitalize">
            {params.tab || "Select a tab"}
          </span>
        </div>
        <div className="h-[93%]">
          {params.tab == "chat" && <ChatSection />}
          {/* {params.tab ? (
              <div className="space-y-2">
                <div className="text-lg font-medium capitalize">
                  {params.tab}
                </div>
                <div className="text-sm">
                  Content for {params.tab} will appear here
                </div>
              </div>
            ) : (
              <div className="text-sm">Select a tab from the sidebar</div>
            )} */}
        </div>
      </div>

      {/* Sidebar Navigation */}
      <TooltipProvider>
        <div className="w-16 bg-muted/30 flex flex-col">
          <div className="flex-1 flex flex-col items-center py-4 space-y-2">
            {options.map((option) => {
              const Icon = option.icon;
              const isActive = params.tab === option.param;

              return (
                <Tooltip key={option.param}>
                  <TooltipTrigger asChild>
                    <Button
                      variant={isActive ? "default" : "ghost"}
                      size="icon"
                      className={cn(
                        "h-10 w-10 transition-all duration-200",
                        isActive
                          ? "bg-primary text-primary-foreground shadow-sm"
                          : "hover:bg-accent hover:text-accent-foreground"
                      )}
                      asChild
                    >
                      <Link
                        to="/arena/$roomId"
                        params={{ roomId }}
                        search={{ tab: option.param }}
                      >
                        <Icon className="h-4 w-4" />
                      </Link>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="left" className="font-medium">
                    {option.title}
                  </TooltipContent>
                </Tooltip>
              );
            })}
          </div>

          <Separator className="mx-2" />

          <div className="p-2">
            <div className="h-8 flex items-center justify-center">
              <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
            </div>
          </div>
        </div>
      </TooltipProvider>
    </div>
  );
};

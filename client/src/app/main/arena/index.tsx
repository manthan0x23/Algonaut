import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Card } from "@/components/ui/card";
import { AlignLeft, Braces } from "lucide-react";
import { useParams } from "@tanstack/react-router";
import { ArenaPanel } from "./_components/panel";
import { Button } from "@/components/ui/button";
import { useEffect } from "react";
import { ThemeSwitch } from "@/integrations/theme/theme-switch";
import { useGetChats } from "./chat/use-get-chats";
import { CollaborativeEditor } from "./editor";
import { WsClient } from "@/integrations/ws";

export const ArenaPage = () => {
  const { roomId } = useParams({
    from: "/arena/$roomId",
  });

  const { isLoading: _chatsLoading } = useGetChats();
  const ws = WsClient.getInstance();

  useEffect(() => {
    ws.connect(roomId);
    return () => {
      ws.disconnect();
    };
  }, [roomId]);

  return (
    <div className="h-screen w-sceen p-4">
      <ThemeSwitch />
      <Card className="h-full w-full overflow-hidden">
        <ResizablePanelGroup direction="horizontal" className="h-full">
          <ResizablePanel defaultSize={70} minSize={50}>
            <div className="h-full flex flex-col">
              <div className="h-12 border-b bg-muted/30 flex items-center justify-between px-4">
                <div className="flex items-center">
                  <AlignLeft className="h-4 w-4 text-muted-foreground" />
                  <span className="ml-2 text-sm font-medium text-muted-foreground">
                    Editor
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <Button variant={"ghost"} size={"icon"}>
                    <Braces size={14} />
                  </Button>
                </div>
              </div>

              <div className="flex-1 bg-background">
                <CollaborativeEditor />
              </div>
            </div>
          </ResizablePanel>

          <ResizableHandle className="w-px bg-border hover:bg-border/80 transition-colors" />

          <ResizablePanel defaultSize={30} maxSize={35} minSize={30}>
            <ArenaPanel />
          </ResizablePanel>
        </ResizablePanelGroup>
      </Card>
    </div>
  );
};

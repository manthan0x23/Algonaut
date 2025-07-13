"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Card } from "@/components/ui/card";
import { defineMinimalMonacoTheme } from "@/integrations/monaco/theme";
import { useThemeStore } from "@/store/theme-store";
import { AlignLeft, Braces } from "lucide-react";
import * as Editor from "@monaco-editor/react";
import { useParams } from "@tanstack/react-router";
import { ArenaPanel } from "./_components/panel";
import { Button } from "@/components/ui/button";
import { useEffect } from "react";
import { WebSocketConnection } from "@/integrations/ws";

export const ArenaPage = () => {
  const { roomId } = useParams({
    from: "/_layout/arena/$roomId",
  });
  const { theme } = useThemeStore();

  const handleBeforeMount = (monaco: any) => {
    defineMinimalMonacoTheme(monaco);
  };

  useEffect(() => {
    const ws = new WebSocketConnection(roomId);

    ws.connect();
  }, [roomId]);

  const monacoTheme = theme === "dark" ? "vs-dark" : "light";

  return (
    <div className="h-full w-full p-4 pt-0">
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
                <Editor.Editor
                  beforeMount={handleBeforeMount}
                  height="100%"
                  defaultLanguage="cpp"
                  defaultValue={roomId}
                  theme={monacoTheme}
                  options={{
                    fontSize: 14,
                    fontFamily:
                      'ui-monospace, SFMono-Regular, "SF Mono", Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
                    minimap: { enabled: false },
                    wordWrap: "on",
                    scrollBeyondLastLine: false,
                    tabSize: 2,
                    lineNumbers: "on",
                    renderWhitespace: "selection",
                    smoothScrolling: true,
                    cursorBlinking: "smooth",
                    padding: { top: 16, bottom: 16 },
                  }}
                />
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

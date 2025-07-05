import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { defineMinimalMonacoTheme } from "@/integrations/monaco/theme";
import { themeStore } from "@/store/theme-store";
import * as Editor from "@monaco-editor/react";
import { useStore } from "@tanstack/react-store";

export const ArenaPage = () => {
  const { theme } = useStore(themeStore);
  const handleBeforeMount = (monaco: any) => {
    defineMinimalMonacoTheme(monaco);
  };

  const monacoTheme = theme == "dark" ? "black-white" : "light";

  return (
    <div className="h-full w-full p-2 pt-0">
      <section className="rounded-lg border h-full w-full overflow-hidden ">
        <ResizablePanelGroup direction="horizontal" className="space-x-2">
          <ResizablePanel defaultSize={50} minSize={25}>
            <div className="h-full w-full">
              <Editor.Editor
                beforeMount={handleBeforeMount}
                height="100%"
                defaultLanguage="cpp"
                defaultValue="// Start solving here..."
                theme={monacoTheme}
                options={{
                  fontSize: 14,
                  minimap: { enabled: false },
                  wordWrap: "on",
                  scrollBeyondLastLine: false,
                  tabSize: 2,
                }}
              />
            </div>
          </ResizablePanel>

          <ResizableHandle withHandle />

          <ResizablePanel defaultSize={50} maxSize={55} minSize={10}>
            <ResizablePanelGroup direction="vertical">
              <ResizablePanel defaultSize={30} minSize={10}>
                <div className="h-full w-full p-6">
                  <p className="font-semibold">Two</p>
                </div>
              </ResizablePanel>

              <ResizableHandle withHandle />

              <ResizablePanel defaultSize={70} minSize={10}>
                <div className="h-full w-full p-6">
                  <p className="font-semibold">Three</p>
                </div>
              </ResizablePanel>
            </ResizablePanelGroup>
          </ResizablePanel>
        </ResizablePanelGroup>
      </section>
    </div>
  );
};

import * as Editor from "@monaco-editor/react";
import { defineMinimalMonacoTheme } from "@/integrations/monaco/theme";
import { useThemeStore } from "@/store/theme-store";
import { useParams } from "@tanstack/react-router";
import { useRef } from "react";
import * as Automerge from "@automerge/automerge";
import { WsClient } from "@/integrations/ws";
import { IncommingCrdtUpdate } from "@/integrations/ws/dtos/incomming";

export const CollaborativeEditor = () => {
  const { roomId } = useParams({ from: "/arena/$roomId" });
  const { theme } = useThemeStore();
  const ws = WsClient.getInstance();

  const editorRef = useRef<any>(null);

  const docRef = useRef(
    Automerge.from<{ content: Automerge.Text }>({
      content: new Automerge.Text(),
    })
  );

  const handleBeforeMount = (monaco: any) => {
    defineMinimalMonacoTheme(monaco);
  };

  const handleOnMount = (editor: any, monaco: any) => {
    editorRef.current = editor;

    const model = editor.getModel();
    model.setValue(docRef.current.content.toString());

    const disposable = model.onDidChangeContent(() => {
      const oldDoc = docRef.current;
      const oldText = oldDoc.content.toString();
      const newText = model.getValue();

      if (oldText === newText) return;

      const newDoc = Automerge.change(oldDoc, (d) => {
        d.content.deleteAt(0, d.content.length);
        d.content.insertAt(0, ...newText.split(""));
      });

      const changes = Automerge.getChanges(oldDoc, newDoc);
      docRef.current = newDoc;

      ws.send({
        type: "crdt",
        update: changes.map((c) => Array.from(c)),
      });
    });

    const onMessage = (msg: any) => {
      const data = IncommingCrdtUpdate.safeParse(msg);
      if (data.success) {
        const msg = data.data;
        const oldDoc = docRef.current;

        console.log("Update ::", msg);

        const binaryChanges = msg.update.map(
          (arr: number[]) => new Uint8Array(arr)
        );
        const [newDoc] = Automerge.applyChanges(oldDoc, binaryChanges);
        docRef.current = newDoc;

        const newValue = newDoc.content.toString();
        if (newValue !== model.getValue()) {
          const position = editor.getPosition();
          model.setValue(newValue);
          if (position) {
            editor.setPosition(position);
          }
        }
      }

      ws.addListener(onMessage);

      console.log(`[CollaborativeEditor] Mounted for room ${roomId}`);

      return () => {
        disposable.dispose();
        ws.removeListener(onMessage);
        console.log(`[CollaborativeEditor] Unmounted for room ${roomId}`);
      };
    };
  };

  const monacoTheme = theme === "dark" ? "vs-dark" : "light";

  return (
    <Editor.Editor
      beforeMount={handleBeforeMount}
      onMount={handleOnMount}
      height="100%"
      defaultLanguage="cpp"
      defaultValue={ws.currentRoom ?? ""}
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
  );
};

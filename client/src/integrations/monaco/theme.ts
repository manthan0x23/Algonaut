// utils/monacoTheme.ts
export const defineMinimalMonacoTheme = (monaco: any) => {
  monaco.editor.defineTheme("black-white", {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "", foreground: "ffffff", background: "000000" }, // default text
    ],
    colors: {
      "editor.background": "#000000",
      "editor.foreground": "#ffffff",
      "editor.lineHighlightBackground": "#111111",
      "editorLineNumber.foreground": "#555555",
      "editorCursor.foreground": "#ffffff",
      "editorIndentGuide.background": "#1a1a1a",
    },
  });
};

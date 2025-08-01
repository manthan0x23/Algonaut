import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { FileTreeAtom } from "@/core/store/atoms/file_tree.atom";
import { DirectoryStructure } from "@/core/types/cde.types";
import { Terminal as XTerminal } from "@xterm/xterm";
import { useEffect, useRef } from "react";
import { useResetRecoilState, useSetRecoilState } from "recoil";
import { Socket, io } from "socket.io-client";
import FileTree from "./FileTree";
import { useLocation, useParams, useSearchParams } from "react-router-dom";
import { FILE_ATOM } from "@/core/store/atoms/file.atom";
import CodeEditor from "./CodeEditor";
import Footer from "./Footer";
import Headers from "./Headers";
import "@xterm/xterm/css/xterm.css";
import { useQuery } from "@tanstack/react-query";
import Server from "@/core/api/api";

const Editor = () => {
  const socketRef = useRef<Socket | null>(null);
  const terminalRef = useRef<HTMLDivElement | null>(null);
  const ports_str = window.localStorage.getItem("ports");

  const { projectId } = useParams();
  const { pathname } = useLocation();
  const [params] = useSearchParams();

  const setFileTree = useSetRecoilState<DirectoryStructure[]>(FileTreeAtom);
  const resetFileTree = useResetRecoilState(FileTreeAtom);
  const setFile = useSetRecoilState(FILE_ATOM);

  const { data } = useQuery({
    queryKey: ["cube", projectId],
    queryFn: () => new Server().cube.run_cube({ cubeId: projectId! }),
    throwOnError: true,
  });

  useEffect(() => {
    async function getfile() {
      const path = params.get("path");
      const res = (await sendRequest("get:file", path!)) as string;
      setFile(res);
    }
    if (socketRef.current) {
      getfile();
    } else {
      console.error("No active socket connection");
    }
  }, [params, pathname]);

  useEffect(() => {
    window.localStorage.setItem("ports", JSON.stringify(data));
    const socket = io(`http://localhost:${data?.ports.express_port}`);

    socket.on("connect", async () => {
      console.log("Connected to Container");

      socketRef.current = socket;

      const message = (await sendRequest("get:fs")) as DirectoryStructure[];
      console.log("Message", message);

      setFileTree(message);

      socket.on("file:tree", (tree) => {
        setFileTree(tree);
      });

      //Terminal

      if (terminalRef.current) {
        const term = new XTerminal({ rows: 20 });

        term.open(terminalRef.current);

        term.onData((data) => {
          socket.emit("terminal:write", data);
        });

        socket.on("terminal:written", (data) => {
          term.write(data);
        });
      }
    });

    window.onbeforeunload = () => {
      disconnect();
    };

    return () => {
      resetFileTree();
      socket.disconnect();
      socketRef.current = null;
    };
  }, []);

  async function disconnect() {
    console.log("Disconnect");

    resetFileTree();
    socketRef.current?.disconnect();
    // new Server().cube.burn_cube({ cubeId: projectId! });
    socketRef.current = null;
  }

  function sendRequest(type: string, data: unknown = {}) {
    return new Promise((resolve, reject) => {
      if (!socketRef.current) {
        alert("No socket state active");
        return;
      }
      socketRef.current.emit(type, data, (response: unknown, err: unknown) => {
        if (!err) {
          resolve(response);
        } else {
          reject(err);
        }
      });
    });
  }

  if (!ports_str) {
    return <div className="h-full w-full">Return no ports active</div>;
  }

  return (
    <div className="h-full w-full flex flex-col justify-center items-center relative">
      <div className="h-[3.5%] w-full bg-primary-black border-b border-gray-500/60">
        <Headers />
      </div>
      <div className=" h-[93%] w-full">
        <ResizablePanelGroup direction="horizontal">
          <ResizablePanel defaultSize={15} maxSize={20} minSize={15}>
            <FileTree />
          </ResizablePanel>
          <ResizableHandle className="bg-white/40" />
          <ResizablePanel defaultSize={55} minSize={45}>
            <ResizablePanelGroup direction="vertical">
              <ResizablePanel defaultSize={70}>
                {socketRef.current && <CodeEditor socket={socketRef.current} />}
              </ResizablePanel>
              <ResizableHandle className="bg-blue-500" />
              <ResizablePanel
                defaultSize={30}
                className="bg-primary-black mb-1"
              >
                {/* Terminal */}
                <div ref={terminalRef} className="overflow-y-scroll p-0 m-0" />
              </ResizablePanel>
            </ResizablePanelGroup>
          </ResizablePanel>
          <ResizableHandle className="bg-gray-500" />
          <ResizablePanel defaultSize={30}>
            <iframe
              src={`http://localhost:${data?.ports.other_port}`}
              className="w-full h-full border-none m-0 p-0 overflow-hidden"
            />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
      <div className="h-[3.5%] w-full bg-primary-black border-t border-gray-500/60">
        <Footer />
      </div>
    </div>
  );
};

export default Editor;

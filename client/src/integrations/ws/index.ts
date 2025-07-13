import { Env } from "@/lib/env";
import { IncomingMessage } from "./dtos/incomming";

type MessageCallback = (msg: IncomingMessage) => void;

export class WebSocketConnection {
  /** 
  16 zeros string with a hash `#0000000000000000` **/
  public static SYSTEM_ID = "#00000000000000000";

  private _socket: null | WebSocket = null;
  private readonly roomId: string;
  private listeners = new Set<MessageCallback>();
  private readonly WS_URL: string;

  constructor(roomId: string) {
    this.roomId = roomId;
    this.WS_URL = Env.ws_url;
  }

  public connect() {
    this._socket = new WebSocket(`${this.WS_URL}/${this.roomId}`);

    this._socket.onopen = () => {
      console.log("[WS] Connected");
    };

    this._socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log(data);
        const parsed = IncomingMessage.safeParse(data);

        if (parsed.success) {
          switch (parsed.data.iden) {
            case "chat":

            case "broadcast":

            case "error":

            case "room_members":

            default:
              break;
          }
        }
      } catch (err) {
        console.error("[WS] Failed to parse message:", err);
      }
    };

    this._socket.onclose = () => {
      console.log("[WS] Disconnected");
    };

    this._socket.onerror = (e) => {
      console.error("[WS] Error:", e);
    };
  }

  public disconnect() {
    if (this._socket) this._socket.close();
  }

  get socket() {
    return this._socket;
  }
}

import { Env } from "@/lib/env";
import type { IncomingMessage } from "./dtos/incomming";

export type MessageCallback = (msg: IncomingMessage) => void;

export class WsClient {
  private static instance: WsClient;
  private socket: WebSocket | null = null;
  private roomId: string | null = null;
  private listeners = new Set<MessageCallback>();
  private reconnectAttempts = 0;
  private maxReconnects = 5;
  private pingInterval: ReturnType<typeof setInterval> | null = null;

  static getInstance() {
    if (!WsClient.instance) {
      WsClient.instance = new WsClient();
    }
    return WsClient.instance;
  }

  connect(roomId: string) {
    if (
      this.socket &&
      this.socket.readyState === WebSocket.OPEN &&
      this.roomId === roomId
    ) {
      console.log(`[WS:${roomId}] Already connected`);
      return;
    }

    this.roomId = roomId;
    const WS_URL = `${Env.ws_url}/${roomId}`;
    console.log(`[WS:${roomId}] Connecting to ${WS_URL}...`);

    this.socket = new WebSocket(WS_URL);

    this.socket.onopen = () => {
      console.log(`[WS:${roomId}] Connected`);
      this.reconnectAttempts = 0;
      this.startHeartbeat();
    };

    this.socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data) as IncomingMessage;
        this.listeners.forEach((cb) => cb(data));
      } catch (err) {
        console.error(`[WS:${roomId}] Failed to parse message:`, err);
      }
    };

    this.socket.onclose = () => {
      console.log(`[WS:${roomId}] Disconnected`);
      this.stopHeartbeat();
      if (this.reconnectAttempts < this.maxReconnects) {
        this.reconnectAttempts++;
        const delay = Math.min(1000 * this.reconnectAttempts, 10000);
        console.log(`[WS:${roomId}] Attempting reconnect in ${delay}ms`);
        setTimeout(() => this.connect(roomId), delay);
      }
    };

    this.socket.onerror = (err) => {
      console.error(`[WS:${roomId}] Error:`, err);
    };
  }

  disconnect() {
    console.log(`[WS] Disconnected manually`);
    this.socket?.close();
    this.stopHeartbeat();
    this.socket = null;
    this.roomId = null;
  }

  send(data: any) {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(data));
    } else {
      console.warn(`[WS] Cannot send, socket not open`);
    }
  }

  addListener(cb: MessageCallback) {
    this.listeners.add(cb);
  }

  removeListener(cb: MessageCallback) {
    this.listeners.delete(cb);
  }

  private startHeartbeat() {
    this.pingInterval = setInterval(() => {
      if (this.socket && this.socket.readyState === WebSocket.OPEN) {
        this.socket.send(JSON.stringify({ type: "ping" }));
      }
    }, 15_000);
  }

  private stopHeartbeat() {
    if (this.pingInterval) {
      clearInterval(this.pingInterval);
      this.pingInterval = null;
    }
  }

  get isConnected() {
    return !!this.socket && this.socket.readyState === WebSocket.OPEN;
  }

  get currentRoom() {
    return this.roomId;
  }
}

export const wsClient = WsClient.getInstance();

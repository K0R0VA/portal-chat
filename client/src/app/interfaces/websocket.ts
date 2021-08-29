import {Observable} from 'rxjs';

export interface IWebsocketService<Response, Request> {
  status: Observable<boolean>;
  on(): Observable<Response>;
  send(data: Request): void;
}

export interface WebSocketConfig {
  url: string;
  binaryType: 'arraybuffer' | 'blob';
  reconnectInterval?: number;
  reconnectAttempts?: number;
}

export interface Serialize {
  encode(): Uint8Array;
}

export interface Deserializer<T> {
  decode(binary: ArrayBuffer): T
}




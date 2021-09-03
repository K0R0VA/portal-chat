import {Inject, Injectable} from '@angular/core';
import {AbstractWebsocket} from '../abstract-websocket/abstract-websocket';
import {IServerMessage, Request, ResponseDeserializer} from '../../../interfaces/message';
import {WebSocketConfig} from '../../../interfaces/websocket';
import {config} from './web-socket.config';

@Injectable({
  providedIn: 'root'
})
export class MainWebsocketService extends AbstractWebsocket<IServerMessage, ResponseDeserializer, Request>{

  public constructor(@Inject(config) protected wsConfig: WebSocketConfig) {
    super(wsConfig);
    this.deserializer = new ResponseDeserializer();
    this.config = {
      url: `${wsConfig.url}/${1}`,
      binaryType: wsConfig.binaryType,
      serializer: (msg: Uint8Array) => {
        const offset = msg.byteOffset;
        const length = msg.byteLength;
        return msg.buffer.slice(offset, offset + length);
      },
      deserializer: msg => new Uint8Array(msg.data as ArrayBuffer),
      closeObserver: {
        next: (event: CloseEvent) => {
          this.websocket$.unsubscribe();
          this.connection$.next(false);
        }
      },
      openObserver: {
        next: (event: Event) => {
          console.log('WebSocket connected!');
          this.connection$.next(true);
        }
      }
    };
    this.connect()
  }
}

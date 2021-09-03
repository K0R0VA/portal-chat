import {Deserializer, Serialize} from './websocket';
import {ClientMessage, ServerMessage} from '../../../messages/message';

export enum MessageType {
  CONTACT = 0,
  GROUP = 1
}

export interface Message {
  readonly senderName: string;
  readonly senderAvatar: string;
  readonly text: string;
}

export interface IClientMessage {
  readonly senderId: number,
  readonly text: string,
  readonly media?: Uint8Array,
  readonly requestType: MessageType,
  readonly recipientId: number,
}

export class Request implements IClientMessage, Serialize {
  encode(): Uint8Array {
    return ClientMessage.encode(this).finish()
  }
  readonly recipientId!: number;
  readonly requestType!: MessageType;
  readonly senderId!: number;
  readonly text!: string;
  readonly media?: Uint8Array;
}

export interface IServerMessage {
  readonly senderId: number,
  readonly text: string,
  readonly requestType: MessageType,
}

export class ResponseDeserializer implements Deserializer<IServerMessage> {
  decode(binary: Uint8Array): IServerMessage {
    const bytes = Array.prototype.slice.call(binary, 0);
    return ServerMessage.decode(bytes as unknown as Uint8Array);
  }
}



import {User} from './user';
import {Message} from './message';

export interface Room {
  readonly id: number,
  readonly name: string,
  readonly avatar?: string
  readonly participants?: User[],
  readonly messages?: Message[]
}

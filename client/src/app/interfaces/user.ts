import {Room} from './room';

export interface User {
  readonly id: number,
  readonly name: string,
  readonly avatar?: string
  readonly rooms?: Room[],
  readonly friends?: User[]
}

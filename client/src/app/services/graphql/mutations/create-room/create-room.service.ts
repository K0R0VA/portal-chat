import { Injectable } from '@angular/core';
import {gql, Mutation} from 'apollo-angular';

@Injectable({
  providedIn: 'root'
})
export class CreateRoomService extends Mutation {
  document = gql`
    mutation ($roomInfo: RoomInfo!) {
      createRoom(roomInfo: $roomInfo) {
        id,
        name,
        avatar,
        participants {
          id,
          name,
          avatar
        }
      }
    }
  `
}

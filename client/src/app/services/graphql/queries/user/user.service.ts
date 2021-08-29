import { Injectable } from '@angular/core';
import {gql, Query} from 'apollo-angular';
import {User} from '../../../../interfaces/user';

@Injectable({
  providedIn: 'root'
})
export class UserGQL extends Query<User, number> {
  document = gql`
    query ($id: Int!) {
      user(id: $id) {
        id,
        name
        avatar,
      }
    }
  `
}

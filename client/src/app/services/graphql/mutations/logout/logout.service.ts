import { Injectable } from '@angular/core';
import {gql, Mutation} from 'apollo-angular';

@Injectable({
  providedIn: 'root'
})
export class LogoutService extends Mutation<number, number> {
  document = gql`
    mutation ($userId: Int!) {
      logout(userId: $userId)
    }
  `
}

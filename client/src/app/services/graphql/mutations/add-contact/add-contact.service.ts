import { Injectable } from '@angular/core';
import {gql, Mutation} from 'apollo-angular';

@Injectable({
  providedIn: 'root'
})
export class AddContactService extends Mutation {
  document = gql`
    mutation ($userId: Int!, $contactId: Int!) {
      addContact(userId: $userId, contactId: $contactId) {
        id,
        name,
        avatar
      }
    }
  `
}

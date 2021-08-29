import { Injectable } from '@angular/core';
import {gql, Mutation} from 'apollo-angular';

@Injectable({
  providedIn: 'root'
})
export class LoadAvatarService extends Mutation {
  document = gql`
    mutation ($userId: Int!, $file: Upload!) {
      loadAvatar(userId: $userId, file: $file)
    }
  `
}

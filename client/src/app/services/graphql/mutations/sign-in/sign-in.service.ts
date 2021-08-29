import {Injectable} from '@angular/core';
import {gql, Mutation} from 'apollo-angular';
import {User} from '../../../../interfaces/user';
import {Icredentials} from '../../../../interfaces/icredentials';

@Injectable({
  providedIn: 'root'
})
export class SignInService extends Mutation<User, Icredentials> {
  document = gql`
    mutation($credentials: Credentials!) {
      signId(credentials: $credentials) {
        id
        name,
        avatar,
        friends {
          id,
          name,
          avatar
        },
        rooms {
          id,
          name,
          avatar
        },
      }
    }
  `
}

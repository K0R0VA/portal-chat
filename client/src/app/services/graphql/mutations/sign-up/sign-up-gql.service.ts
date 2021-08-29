import { Injectable } from '@angular/core';
import {gql, Mutation} from 'apollo-angular';
import {Icredentials} from '../../../../interfaces/icredentials';
import {User} from '../../../../interfaces/user';

@Injectable({
  providedIn: 'root'
})
export class SignUpService extends Mutation<User, Icredentials> {
  document = gql`
    mutation($credentials: Credentials!) {
      signUp(credentials: $credentials) {
        id,
        name,
      }
    }
  `
}

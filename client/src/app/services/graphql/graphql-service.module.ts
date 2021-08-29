import {NgModule} from '@angular/core';
import {UserGQL} from './queries/user/user.service';
import {SignInService} from './mutations/sign-in/sign-in.service';
import {SignUpService} from './mutations/sign-up/sign-up-gql.service';
import {LogoutService} from './mutations/logout/logout.service';
import {GraphQLModule} from '../../graphql.module';

@NgModule({
  imports: [GraphQLModule],
  providers: [UserGQL, SignInService, SignUpService, LogoutService]
})

export class GraphqlServiceModule {}

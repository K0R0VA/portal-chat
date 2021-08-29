import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterModule} from '@angular/router';
import {AuthenticationService} from './authentication.service';
import {UserModule} from '../user/user.module';
import {GraphqlServiceModule} from '../graphql/graphql-service.module';

@NgModule({
  imports: [
    CommonModule,
    RouterModule,
    GraphqlServiceModule,
    UserModule
  ],
  providers: [AuthenticationService]
})

export class AuthModule {}

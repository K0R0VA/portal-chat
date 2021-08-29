import {NgModule} from '@angular/core';
import {UserService} from './user.service';
import {CommonModule} from '@angular/common';
import {GraphqlServiceModule} from '../graphql/graphql-service.module';

@NgModule({
  imports: [CommonModule, GraphqlServiceModule],
  providers: [UserService]
})

export class UserModule {}

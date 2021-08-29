import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {MessageListComponent} from './message-list.component';
import {MessageModule} from '../message/message.module';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    MessageModule,
  ],
  providers: [],
  declarations: [MessageListComponent],
  exports: [MessageListComponent]
})

export class MessageListModule {}

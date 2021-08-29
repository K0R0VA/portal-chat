import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {ChatContentComponent} from './chat-content.component';
import {MessageListModule} from '../message-list/message-list.module';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    MessageListModule
   ],
  providers: [],
  declarations: [ChatContentComponent],
  exports: [ChatContentComponent]
})

export class ChatContentModule {}

import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {ChatContentModule} from '../chat-content/chat-content.module';
import {ChatComponent} from './chat.component';
import {ChatFooterModule} from '../chat-footer/chat-footer.module';
import {ChatHeaderModule} from '../chat-header/chat-header.module';
import {FormsModule} from '@angular/forms';
import {DirectivesModule} from '../../../directives/directives.module';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    FormsModule,
    ChatHeaderModule,
    ChatContentModule,
    ChatFooterModule,
    DirectivesModule
  ],
  providers: [],
  declarations: [ChatComponent],
  exports: [ChatComponent]
})

export class ChatModule {}

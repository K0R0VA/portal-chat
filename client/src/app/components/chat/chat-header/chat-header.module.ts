import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {ChatHeaderComponent} from './chat-header.component';

@NgModule({
  imports: [
    CommonModule,
    IonicModule
  ],
  providers: [],
  declarations: [ChatHeaderComponent],
  exports: [ChatHeaderComponent]
})

export class ChatHeaderModule {}

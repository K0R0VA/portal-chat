import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {ChatFooterComponent} from './chat-footer.component';
import {FormsModule} from '@angular/forms';
import {DirectivesModule} from '../../../directives/directives.module';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    FormsModule,
    DirectivesModule
  ],
  providers: [],
  declarations: [ChatFooterComponent],
  exports: [ChatFooterComponent]
})

export class ChatFooterModule {
}

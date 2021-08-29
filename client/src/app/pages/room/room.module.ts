import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonicModule } from '@ionic/angular';
import {RoomPage} from './room';
import {ChatModule} from '../../components/chat/chat/chat.module';
import {RoomRoutingModule} from './room-routing.module';
import {DirectivesModule} from '../../directives/directives.module';


@NgModule({
  imports: [
    CommonModule,
    DirectivesModule,
    FormsModule,
    IonicModule,
    RoomRoutingModule,
    ChatModule
  ],
  declarations: [RoomPage]
})
export class RoomModule { }

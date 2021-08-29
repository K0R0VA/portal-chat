import {ModuleWithProviders, NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {WebSocketConfig} from '../../../interfaces/websocket';
import { config } from './web-socket.config';
import {MainWebsocketService} from './main-websocket.service';

@NgModule({
  imports: [
    CommonModule
  ],
  providers: [
    MainWebsocketService
  ]
})
export class MainWebSocketModule {
  public static config(wsConfig: WebSocketConfig): ModuleWithProviders<unknown> {
    return {
      ngModule: MainWebSocketModule,
      providers: [{provide: config, useValue: wsConfig }]
    };
  }
}

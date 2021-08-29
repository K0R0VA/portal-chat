import {NgModule} from '@angular/core';
import {IonicModule} from '@ionic/angular';
import {CommonModule} from '@angular/common';
import {SidebarHeaderComponent} from './sidebar-header.component';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
  ],
  declarations: [SidebarHeaderComponent],
  exports: [SidebarHeaderComponent]
})

export class SidebarHeaderModule {}

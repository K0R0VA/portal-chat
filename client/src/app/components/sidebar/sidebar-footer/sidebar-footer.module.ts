import {NgModule} from '@angular/core';
import {IonicModule} from '@ionic/angular';
import {CommonModule} from '@angular/common';
import {RouterModule} from '@angular/router';
import {SidebarFooterComponent} from './sidebar-footer.component';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    RouterModule
  ],
  declarations: [SidebarFooterComponent],
  exports: [SidebarFooterComponent],
  providers: []
})

export class SidebarFooterModule {}

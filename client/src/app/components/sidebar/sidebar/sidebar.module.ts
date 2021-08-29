import {NgModule} from '@angular/core';
import {SidebarComponent} from './sidebar.component';
import {CommonModule} from '@angular/common';
import {SidebarFooterModule} from '../sidebar-footer/sidebar-footer.module';
import {NavigationModule} from '../navigation/navigation/navigation.module';
import {SidebarHeaderModule} from '../sidebar-header/sidebar-header.module';
import {IonicModule} from '@ionic/angular';
import {RouterModule} from '@angular/router';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    RouterModule,
    SidebarFooterModule,
    SidebarHeaderModule,
    NavigationModule,
  ],
  exports: [SidebarComponent],
  declarations: [SidebarComponent]
})

export class SidebarModule {}

import {NgModule} from '@angular/core';
import {NavigationComponent} from './navigation.component';
import {IonicModule} from '@ionic/angular';
import {CommonModule} from '@angular/common';
import {NavigationItemModule} from '../navigation-item/navigation-item.module';
import {RouterModule} from '@angular/router';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    RouterModule,
    NavigationItemModule
  ],
  providers: [],
  declarations: [NavigationComponent],
  exports: [NavigationComponent],
})

export class NavigationModule {}

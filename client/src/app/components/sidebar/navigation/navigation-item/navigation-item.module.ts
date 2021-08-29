import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {IonicModule} from '@ionic/angular';
import {RouterModule} from '@angular/router';
import {NavigationItemComponent} from './navigation-item.component';

@NgModule({
  imports: [
    CommonModule,
    IonicModule,
    RouterModule
  ],
  declarations: [NavigationItemComponent],
  exports: [NavigationItemComponent]
})

export class NavigationItemModule {}

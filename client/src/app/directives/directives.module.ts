import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {AbsoluteDirective} from './absolute.directive';
import { BottomDirective } from './bottom.directive';

@NgModule({
  imports: [CommonModule],
  providers: [],
  declarations: [AbsoluteDirective, BottomDirective],
  exports: [AbsoluteDirective, BottomDirective]
})

export class DirectivesModule {}

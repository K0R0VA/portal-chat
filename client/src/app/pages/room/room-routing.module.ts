import {RouterModule, Routes} from '@angular/router';
import {RoomPage} from './room';
import {NgModule} from '@angular/core';

const routes: Routes = [
  {
    path: '',
    component: RoomPage
  }
]

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})

export class RoomRoutingModule {}

import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {AuthorizeGuard} from './guards/authorize-guard.service';

const routes: Routes = [
  {
    path: 'account',
    canActivate: [AuthorizeGuard],
    loadChildren: () => import('./pages/account/account.module').then(m => m.AccountModule)
  },
  {
    path: 'room/:roomId',
    canActivate: [AuthorizeGuard],
    loadChildren: () => import('./pages/room/room.module').then(m => m.RoomModule)
  },
  {
    path: 'login',
    loadChildren: () => import('./pages/login/login.module').then(m => m.LoginModule)
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, { relativeLinkResolution: 'legacy' })],
  exports: [RouterModule]
})
export class AppRoutingModule {}

import { Injectable } from '@angular/core';
import {CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router} from '@angular/router';
import { Observable } from 'rxjs';
import {AuthenticationService} from '../services/auth/authentication.service';

@Injectable({
  providedIn: 'root'
})
export class AuthorizeGuard implements CanActivate {
  private isAuthenticated: boolean = false;
  constructor(private authService: AuthenticationService, private router: Router) {
    this.authService.isAuthenticated.subscribe(value => {
      this.isAuthenticated = value;
    })
  }
  canActivate(
    next: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {
    if (this.isAuthenticated) {
      return true;
    }
    this.router.navigate(['/login']).then()
    return false;
  }
}

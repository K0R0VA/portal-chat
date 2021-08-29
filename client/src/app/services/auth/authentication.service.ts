import {Injectable, OnDestroy} from '@angular/core';
import {SignUpService} from '../graphql/mutations/sign-up/sign-up-gql.service';
import {Icredentials} from '../../interfaces/icredentials';
import {SignInService} from '../graphql/mutations/sign-in/sign-in.service';
import {BehaviorSubject, Observable, Subscription} from 'rxjs';
import {map} from 'rxjs/operators';
import {User} from '../../interfaces/user';
import {Router, RouterLink} from '@angular/router';
import {LogoutService} from '../graphql/mutations/logout/logout.service';
import {UserService} from '../user/user.service';
import {CookieService} from 'ngx-cookie-service';

@Injectable({
  providedIn: 'root'
})
export class AuthenticationService implements OnDestroy {

  isAuthenticated: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  private readonly subscription: Subscription;
  private userId: number;

  constructor(
    private signUpService: SignUpService,
    private signInService: SignInService,
    private logoutService: LogoutService,
    private userService: UserService,
    private router: Router,
  ) {
    const id = localStorage.getItem('id')
    if (id) {
      this.isAuthenticated.next(true);
      this.subscription = this.userService.getUser().subscribe(user => {
        this.userId = user.id;
      });
    }
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  signUp(credentials: Icredentials) {
    this.signUpService
      .mutate(credentials)
      .subscribe(async ({data}) => {
        this.isAuthenticated.next(true);
        this.userService.setUser(data);
        await this.router.navigate(['']);
      });
  }

  signIn(credentials: Icredentials) {
    this.signInService.mutate(credentials)
      .subscribe(async ({data}) => {
        this.isAuthenticated.next(true);
        this.userService.setUser(data);
        await this.router.navigate(['']);
      });
  }

  logout() {
    this.logoutService
      .mutate(this.userId)
      .subscribe(() => {
        this.isAuthenticated.next(false);
        this.userService.clear()
      });
  }

}

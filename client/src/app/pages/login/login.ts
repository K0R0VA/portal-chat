import {Component} from '@angular/core';
import {AuthenticationService} from '../../services/auth/authentication.service';

@Component({
  selector: 'page-login',
  templateUrl: 'login.html',
})
export class LoginPage {
  submitted = false;
  login: string = '';
  password: string = '';

  signUp() {
    this.authenticationService.signUp({
      login: this.login,
      password: this.password
    });
  }

  signIn() {
    this.authenticationService.signIn({
      login: this.login,
      password: this.password
    });
  }


  constructor(
    private authenticationService: AuthenticationService
  ) {

  }
}

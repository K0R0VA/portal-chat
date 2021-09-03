import { AfterViewInit, Component } from '@angular/core';
import { Router } from '@angular/router';

import { AlertController } from '@ionic/angular';


@Component({
  selector: 'page-account',
  templateUrl: 'account.html',
})
export class AccountPage implements AfterViewInit {
  ngAfterViewInit(): void {
  }
  username!: string;

  constructor(
    public alertCtrl: AlertController,
    public router: Router,
  ) { }
}

import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-sidebar-footer',
  templateUrl: './sidebar-footer.component.html',
})

export class SidebarFooterComponent implements OnInit {
  loggedIn = false;

  constructor() { }

  ngOnInit() {}

  logout() {}

}

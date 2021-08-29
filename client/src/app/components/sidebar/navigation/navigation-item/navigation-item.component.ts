import {Component, Input, OnInit} from '@angular/core';
import {Chat} from '../../../../interfaces/chat';

@Component({
  selector: 'app-navigation-item',
  templateUrl: './navigation-item.component.html',
})
export class NavigationItemComponent implements OnInit {
  @Input() chat: Chat;
  constructor() { }

  ngOnInit() {}
}

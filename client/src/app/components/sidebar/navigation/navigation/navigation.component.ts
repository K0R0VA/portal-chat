import {Component, Input, OnInit} from '@angular/core';
import {Observable} from 'rxjs';
import {Chat} from '../../../../interfaces/chat';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
})
export class NavigationComponent implements OnInit {
  @Input() chats: Observable<Chat[]> = new Observable();

  constructor() { }

  ngOnInit() {}

}

import {Component, OnInit} from '@angular/core';
import {NavigationComponent} from '../navigation/navigation/navigation.component';
import {from, Observable} from 'rxjs';
import {Chat} from '../../../interfaces/chat';

@Component({
  selector: 'app-sidebar',
  templateUrl: './sidebar.component.html',
  providers: [NavigationComponent,]
})
export class SidebarComponent implements OnInit {
  public chats: Observable<Chat[]> = new Observable();

  constructor() {
  }

  ngOnInit() {
    const chats: Chat[] = [
      {
        id: 1,
        name: 'комната'
      },
      {
        id: 2,
        name: 'вторая комната'
      }
    ];
    this.chats = new Observable<Chat[]>(subscriber => {
     subscriber.next(chats)
    });
  }

}

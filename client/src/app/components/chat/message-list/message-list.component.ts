import {Component, Input, OnInit} from '@angular/core';
import {Message} from '../../../interfaces/message';

@Component({
  selector: 'app-message-list',
  templateUrl: './message-list.component.html',
})
export class MessageListComponent implements OnInit {
  @Input() messages: Message[];

  constructor() { }

  ngOnInit() {
  }

}

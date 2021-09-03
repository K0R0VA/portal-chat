import {Component, Input, OnInit} from '@angular/core';
import {Message} from '../../../interfaces/message';

@Component({
  selector: 'app-message',
  templateUrl: './message.component.html',
})
export class MessageComponent implements OnInit {
  @Input() message!: Message;

  constructor() { }

  ngOnInit() {
  }

}

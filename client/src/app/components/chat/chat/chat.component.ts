import {Component, Input, OnInit, Output} from '@angular/core';
import { EventEmitter } from '@angular/core';

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
})

export class ChatComponent implements OnInit {
  @Input() title: string = "Комната";
  @Output() messageSubmit: EventEmitter<string> = new EventEmitter<string>();

  ngOnInit() {}

  submit($event: string) {
    this.messageSubmit.emit($event);
  }
}

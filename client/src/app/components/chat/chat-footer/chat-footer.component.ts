import {Component, OnInit, Output} from '@angular/core';
import { EventEmitter } from '@angular/core';

@Component({
  selector: 'app-chat-footer',
  templateUrl: './chat-footer.component.html',
})

export class ChatFooterComponent implements OnInit {
  @Output() messageSubmit: EventEmitter<string> = new EventEmitter();
  text: string;
  constructor() { }

  ngOnInit() {}

  submit() {
    this.messageSubmit.emit(this.text);
  }
}

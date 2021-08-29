import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'app-chat-header',
  templateUrl: './chat-header.component.html',
})
export class ChatHeaderComponent implements OnInit {
  @Input() title: string;
  constructor() { }

  ngOnInit() {}

}

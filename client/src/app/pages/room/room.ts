import { Component, OnInit } from '@angular/core';
import {Router} from '@angular/router';

@Component({
  selector: 'app-room',
  templateUrl: './room.html',
})
export class RoomPage implements OnInit {

  constructor(private router: Router) { }

  ngOnInit() {}

}

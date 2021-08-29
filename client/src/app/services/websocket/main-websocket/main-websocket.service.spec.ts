import { TestBed } from '@angular/core/testing';

import { MainWebsocketService } from './main-websocket.service';

describe('MainWebsocketService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: MainWebsocketService = TestBed.get(MainWebsocketService);
    expect(service).toBeTruthy();
  });
});

import { TestBed } from '@angular/core/testing';

import { StreamingWebsocketService } from './streaming-websocket.service';

describe('StreamingWebsocketService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: StreamingWebsocketService = TestBed.get(StreamingWebsocketService);
    expect(service).toBeTruthy();
  });
});

import { TestBed } from '@angular/core/testing';

import { CreateRoomService } from './create-room.service';

describe('CreateRoomService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: CreateRoomService = TestBed.get(CreateRoomService);
    expect(service).toBeTruthy();
  });
});

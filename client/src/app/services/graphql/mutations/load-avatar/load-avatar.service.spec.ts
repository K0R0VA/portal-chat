import { TestBed } from '@angular/core/testing';

import { LoadAvatarService } from './load-avatar.service';

describe('LoadAvatarService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: LoadAvatarService = TestBed.get(LoadAvatarService);
    expect(service).toBeTruthy();
  });
});

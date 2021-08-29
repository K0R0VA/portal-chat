import { TestBed } from '@angular/core/testing';

import { AddContactService } from './add-contact.service';

describe('AddContactService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: AddContactService = TestBed.get(AddContactService);
    expect(service).toBeTruthy();
  });
});

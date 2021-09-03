import {Injectable, OnDestroy} from '@angular/core';
import {BehaviorSubject, Observable, Subscription} from 'rxjs';
import {User} from '../../interfaces/user';
import {Option, none, some} from 'fp-ts/Option';
import {UserGQL} from '../graphql/queries/user/user.service';

@Injectable({
  providedIn: 'root'
})
export class UserService implements OnDestroy {

  private user = new BehaviorSubject<Option<User>>(none);
  private subscription: Subscription = new Subscription();

  constructor(private userQueryService: UserGQL) {
    let id = localStorage.getItem('user');
    if (id) {
      this.subscription = this.userQueryService
        .fetch(Number(id))
        .subscribe(({data}) => {
          this.user.next(some(data));
        });
    }
  }

  getUser(): Observable<Option<User>> {
    return this.user.asObservable();
  }

  setUser(user: User): void {
    localStorage.setItem('user', String(user.id));
    this.user.next(some(user));
  }

  clear(): void {
    this.user.next(none);
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

}

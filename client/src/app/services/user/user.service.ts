import {Injectable, OnDestroy} from '@angular/core';
import {BehaviorSubject, Observable, Subscription} from 'rxjs';
import {User} from '../../interfaces/user';
import {UserGQL} from '../graphql/queries/user/user.service';

@Injectable({
  providedIn: 'root'
})
export class UserService implements OnDestroy {

  private user: BehaviorSubject<User> = new BehaviorSubject<User>(null);
  private subscription: Subscription;

  constructor(private userQueryService: UserGQL) {
    let id = localStorage.getItem('user');
    if (id) {
      this.subscription = this.userQueryService
        .fetch(Number(id))
        .subscribe(({data}) => {
          this.user.next(data);
        });
    }
  }

  getUser(): Observable<User> {
    return this.user.asObservable();
  }

  setUser(user: User) {
    localStorage.setItem('user', String(user.id));
    this.user.next(user);
  }

  clear() {
    this.user.next(null);
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

}

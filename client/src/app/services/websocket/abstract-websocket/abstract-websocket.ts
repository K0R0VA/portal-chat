import {Inject, OnDestroy} from '@angular/core';
import {Deserializer, IWebsocketService, Serialize, WebSocketConfig} from '../../../interfaces/websocket';
import {interval, Observable, Observer, Subject, SubscriptionLike} from 'rxjs';
import {WebSocketSubject, WebSocketSubjectConfig} from 'rxjs/internal-compatibility';
import {config} from '../main-websocket/web-socket.config';
import {distinctUntilChanged, map, share, takeWhile} from 'rxjs/operators';

@Inject({
  providedIn: 'root'
})
export abstract class AbstractWebsocket
  <
    ResponseMessage,
    ResponseDeserializer extends Deserializer<ResponseMessage>,
    Request extends Serialize
  >
  implements IWebsocketService<ResponseMessage, Request>, OnDestroy {

  protected config!: WebSocketSubjectConfig<Uint8Array>;
  protected websocket$!: WebSocketSubject<Uint8Array>;
  protected deserializer!: ResponseDeserializer;
  private websocketSub: SubscriptionLike;
  private statusSub: SubscriptionLike;

  private reconnection$!: Observable<number>;

  protected connection$!: Observer<boolean>;
  private wsMessages$: Subject<ResponseMessage>;

  private reconnectInterval: number;
  private readonly reconnectAttempts: number;
  private isConnected!: boolean;

  public status: Observable<boolean>;

  protected constructor(@Inject(config) protected wsConfig: WebSocketConfig) {
    this.wsMessages$ = new Subject<ResponseMessage>();

    this.reconnectInterval = wsConfig.reconnectInterval || 5000; // pause between connections
    this.reconnectAttempts = wsConfig.reconnectAttempts || 10; // number of connection attempts

    this.status = new Observable<boolean>((observer) => {
      this.connection$ = observer;
    }).pipe(share(), distinctUntilChanged());

    this.statusSub = this.status
      .subscribe((isConnected) => {
        this.isConnected = isConnected;
        if (!this.reconnection$ && typeof (isConnected) === 'boolean' && !isConnected) {
          this.reconnect();
        }
      });
    this.websocketSub = this.wsMessages$.subscribe(
      null, (error: ErrorEvent) => console.error('WebSocket error!', error)
    );
  }

  protected connect(): void {
    this.websocket$ = new WebSocketSubject(this.config);

    this.websocket$.subscribe(
      (event) => {
        let message = this.deserializer.decode(event);
        this.wsMessages$.next(message);
      },
      (error: Event) => {
        if (!this.websocket$) {
          this.reconnect();
        }
      });
  }

  private reconnect(): void {
    this.reconnection$ = interval(this.reconnectInterval)
      .pipe(takeWhile((v, index) => index < this.reconnectAttempts && !this.websocket$));

    this.reconnection$.subscribe(
      () => this.connect(),
      null,
      () => {
        // Subject complete if reconnect attemts ending
        this.reconnection$ = null as unknown as Observable<number>;

        if (!this.websocket$) {
          this.wsMessages$.complete();
          this.connection$.complete();
        }
      });
  }

  public on(): Observable<ResponseMessage> {
    return this.wsMessages$.pipe(
      map((message: ResponseMessage) => message)
    );
  }

  public send(data: Request): void {
    if (this.isConnected) {
      this.websocket$.next(data.encode());
    }
  }

  ngOnDestroy(): void {
    this.websocketSub.unsubscribe();
    this.statusSub.unsubscribe();
    this.deserializer = null as unknown as ResponseDeserializer;
  }
}

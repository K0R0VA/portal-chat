import {HttpClientModule} from '@angular/common/http';
import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';
import {SplashScreen} from '@ionic-native/splash-screen/ngx';
import {StatusBar} from '@ionic-native/status-bar/ngx';
import {IonicModule} from '@ionic/angular';
import {IonicStorageModule} from '@ionic/storage';
import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {ServiceWorkerModule} from '@angular/service-worker';
import {environment} from '../environments/environment';
import {FormsModule} from '@angular/forms';
import {GraphQLModule} from './graphql.module';
import {SidebarModule} from './components/sidebar/sidebar/sidebar.module';
import {MainWebSocketModule} from './services/websocket/main-websocket/main-websocket.module';
import {DirectivesModule} from './directives/directives.module';
import {AuthModule} from './services/auth/auth.module';
import {UserModule} from './services/user/user.module';
import {GraphqlServiceModule} from './services/graphql/graphql-service.module';

@NgModule({
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    FormsModule,
    IonicModule.forRoot(),
    IonicStorageModule.forRoot(),
    ServiceWorkerModule.register('ngsw-worker.js', {
      enabled: environment.production,
    }),
    GraphQLModule,
    GraphqlServiceModule,
    // @ts-ignore
    MainWebSocketModule.config({
      url: environment.ws,
      binaryType: 'arraybuffer',
    }),
    AuthModule,
    UserModule,
    SidebarModule,
    DirectivesModule
  ],
  declarations: [AppComponent],
  providers: [SplashScreen, StatusBar],
  bootstrap: [AppComponent]
})
export class AppModule {
}

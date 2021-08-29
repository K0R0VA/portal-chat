import {NgModule} from '@angular/core';
import {APOLLO_OPTIONS} from 'apollo-angular';
import {ApolloClientOptions, InMemoryCache} from '@apollo/client/core';
import {HttpLink} from 'apollo-angular/http';

const uri = 'https://192.168.0.7:8081/graphql'; // <-- add the URL of the GraphQL server here

@NgModule({
  providers: [
    {
      provide: APOLLO_OPTIONS,
      useFactory: (httpLink: HttpLink): ApolloClientOptions<any> => {
      return {
        link: httpLink.create({uri}),
        cache: new InMemoryCache(),
      };
    },
      deps: [HttpLink],
    },
  ],
})
export class GraphQLModule {}

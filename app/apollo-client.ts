import { ApolloClient, HttpLink, InMemoryCache } from "@apollo/client";
// import { createPersistedQueryLink, PersistedQueryLink } from "@apollo/client/link/persisted-queries";
// import {sha256} from 'crypto-hash'

// const linkchain = createPersistedQueryLink({sha256}).concat(
//     new HttpLink({ uri: "http://localhost:4000/graphql"}));

const client = new ApolloClient({ 
    uri: "http://localhost:4000/",
    cache: new InMemoryCache,
    // link: linkchain
});

export default client
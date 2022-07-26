import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { AuthProvider } from '../hooks/useAuth'
import { RecoilRoot } from 'recoil'
import { ApolloProvider } from '@apollo/client'
import client from '../apollo-client'


/*
  The ApolloProvider component uses React's Context API to make a configured Apollo client
  available everywhere it's needed
*/

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ApolloProvider client ={client}>
      <RecoilRoot>
        <AuthProvider>
          <Component {...pageProps} />
        </AuthProvider>
      </RecoilRoot>
    </ApolloProvider>   
  )
}

export default MyApp

import Head from 'next/head';
import Image from 'next/image';
import Banner from '../components/banner';
import Header from '../components/header';
import requests from '../utils/request';
import { Movie, MovieType } from '../typings';
import Row from '../components/row';
import useAuth from '../hooks/useAuth';
import { useRecoilValue } from 'recoil';
import { modalState, movieState } from '../atoms/model';
import Model from '../components/model'
import Plans from '../components/plans';
import { getProducts, Product } from '@stripe/firestore-stripe-payments';
import payments from '../lib/stripe';
import useSubscription from '../hooks/useSubscription';
import use_list from '../hooks/use_list';

import { gql, useQuery } from "@apollo/client";
import client from "../apollo-client";
import { useState } from 'react';
import Search from './search';
import { FilteredSearch, FilteredSearchResult, FilteredSearchVariables, FILTERED_QUERY } from '../utils/filtered_query';
import { ServerRequest, SortedSearchResult, SortedSearchVariables, SORTED_QUERY } from '../utils/sort_by_query';

interface MovieProps { 
  top_trending: MovieType[],
  netflix_originals: Movie[],
  products: Product[],
  action_movies: MovieType[],
  horror_movies: MovieType[],
  romantic_movies: MovieType[],
  documentaries_movies: MovieType[],
  comedy_movies: MovieType[],
  science_fiction: MovieType[],
  crime_fiction: MovieType[],
  top_rated: MovieType[]
}


const Home = ({ 
  netflix_originals,
  products,
  action_movies,
  horror_movies,
  romantic_movies,
  documentaries_movies,
  comedy_movies,
  science_fiction,
  crime_fiction,
  top_trending,
  top_rated
}: MovieProps) => {
  console.log(netflix_originals);
  console.log(products);
  const { loading, user } = useAuth();
  const showModel = useRecoilValue(modalState);
  const movie = useRecoilValue(movieState);
  const subscription = useSubscription(user);
  const list = use_list(user?.uid);


  console.log("MOVIES: ==> {}", action_movies)

  const [first, setFirst] = useState();
  //  Add Loading ui
  if (loading || subscription === null) { return null }

  if (!subscription) { 
    return <Plans products={products}/>
  }
  
  const top: MovieType[] = top_trending.filter((movie) => movie.poster );

  // console.log("  ACCCOUNTSSSS ============  $1", accounts)
  console.log(subscription);
  console.log("asdasdasd   $1", showModel);
  console.log("This is the movie $1", movie);

  return (
    <div className="relative h-screen bg-gradient-to-b from-gray-900/10
     to-[#010511] lg:h-[140vh]">
      <Head>
        <title>Netflix</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Header/>
      <main className='relative pl-5 pb-24 lg:space-y-24 lg:pl-16'>
        <Banner netflix_original={netflix_originals}/>
        <section className="space-y-24">
          {/* MY LIST  */}
          {
            list.length > 0 && <Row title="My list" movies={science_fiction}/>
          }
      
          <Row title="Trending Now" movies={top}/> 
          <Row title="Top Rated" movies={top_rated}/> 
          <Row title="Action Thrillers" movies={action_movies}/>
          <Row title="Comedies" movies={comedy_movies}/>
          <Row title="Scary Movies" movies={horror_movies}/>
          <Row title="Rom-Coms" movies={romantic_movies}/>
          <Row title="Documentaries" movies={documentaries_movies}/>
          <Row title="Sci-Fi" movies={science_fiction}/>
          <Row title="Crime" movies={crime_fiction}/>
        </section>
        

      </main>
      
      { showModel && <Model/>}
    </div>
  )
}

export default Home

export const getServerSideProps = async () => { 
  // Serverside grapqhl queries 
 const [
  action_movies,
  crime_fiction,
  comedy_movies,
  documentaries_movies,
  science_fiction,
  horror_movies,
  romantic_movies,
 ] = await Promise.all([
  (await FilteredSearch.action_movies).data.filterBy,
  (await FilteredSearch.crime_fiction).data.filterBy,
  (await FilteredSearch.comedy_movies).data.filterBy,
  (await FilteredSearch.documentaries_movies).data.filterBy,
  (await FilteredSearch.science_fiction).data.filterBy,
  (await FilteredSearch.horror_movies).data.filterBy,
  (await FilteredSearch.romantic_movies).data.filterBy,
  
 ])
 
  const products = await getProducts(payments, { 
    includePrices: true,
    activeOnly: true
  })
    .then((resp) => resp)
    .catch((e) => console.log(e.message));

  // Graphql queries
  const [
    top_trending,
    top_rated
  ] = await Promise.all([
    (await ServerRequest.top_trending).data.sortMoviesAccordingly,
    (await ServerRequest.top_rated).data.sortMoviesAccordingly,

  ])

  const [
    netflix_originals,
  ] = await Promise.all([
    fetch(requests.fetchTopRated).then((resp) => resp.json()),
  ])

  return { 
    props: { 
      netflix_originals: netflix_originals.results,
      products,
      action_movies,
      horror_movies,
      romantic_movies,
      documentaries_movies,
      comedy_movies,
      science_fiction,
      crime_fiction,
      top_trending,
      top_rated
    }
  }
}

import React, { useState } from 'react'
import { gql, useLazyQuery, useQuery } from '@apollo/client';
import {useCombobox, resetIdCounter} from 'downshift'
import { useRouter } from 'next/router';
import debounce from 'lodash.debounce';
import {MovieType} from '../typings'
import ThumbNail from '../components/thumbnail';
import Header from '../components/header';
import Head from 'next/head';
import Banner from '../components/banner';
import Row from '../components/row';
import { SearchIcon } from '@heroicons/react/outline';
import Col from '../components/col';
import { AggregatedQuery, SearchAllResult, SearchAllVariables,  SearchInputs, SearchTextInput, SEARCH_ALL, SEARCH_MOVIE } from '../utils/search_result';
import Model from '../components/model';
import {  modalState } from '../atoms/model';
import FilterBy from '../components/filterby'
import { useRecoilValue } from 'recoil';
import FilterMenu from '../components/filter_menu'


function Search() {
  const [Search, setSearch] = useState<string>('')
  const [searchInUse, SearchInUse] = useState(false)
  // Popup
  const showModel = useRecoilValue(modalState);
 
  
  /*
    Show total fields:
    By default it is '30'
  */
  const [totalResult, setTotalResult] = useState<number | null>(null)
  /*
    Sorting:
    By default, 
      sort_by is defaulted to 'ratings.popularity'
      order is defaulted to 'asc'
  */  
  const [sortBy, setSortBy] = useState<string | null>(null);
  const [order, setOrder] = useState<string | null>(null);

  /*
    Aggregation By 
    By default,
      AggField is set to empty string,
      AggSize is set to 1
  */
  const [aggField, setAggField] = useState<string | null>(null);
  const [aggSize, setAggSize] = useState(1)


  const [filterBy, setFilterBy] = useState<string | null>(null);
  const [filterValue, setFilterValue] = useState<string | null>(null);

  // const genre = useRecoilValue(genreState);
  
  
  const {data} = useQuery<
    {searchMovie: AggregatedQuery}, 
    {input: SearchInputs}
    >(SEARCH_MOVIE, {
      variables: {
        input: {
          query: Search,
          indexName: "movies",
          sortBy: sortBy,
          order: order,
          aggField: "genres.keyword",
          aggSize: 5,
          totalResult: totalResult,
          filterBy: "genre",
          filterValue
        }
      }
    })

  // Drop down menu options 
  const sort_by_most_popular = () =>  {
    setSortBy("rating.popularity");
    setOrder("asc")
  }

  const sort_by_least_popular = () => { 
    setSortBy("rating.popularity");
    setOrder("desc")
  }

  const index: number | undefined = data?.searchMovie?.movieList?.filter(movie => movie.keywords?.length !== 0)?.findIndex(i => i);
  console.log("THIS IS THE INDESX", index)
  const related_words = data?.searchMovie?.movieList[index!]?.keywords?.filter((id, val) => val < 5)
    .map(word => (
      <div>{word}</div>
    ));
  

  //  DOWNSHIFT EEXA
  //  Check if the values exists --> this is coming from apollo router
  // console.log("The movie Data", data);
  // console.log("is it loading", loading);
  // const router = useRouter();
  // const [searchItems, {loading, data, called}]  = useLazyQuery(
  //   SEARCH_MOVIE_QUERY, 
  //   {
  //     fetchPolicy: 'no-cache'
  //     // variables: { 
  //     //   totalResult: 10,
  //     //   indexName: "movies",
  //     //   query: Search 
  //     // }
  //   }
  // );
  // const [inputItems, setInputItems] = useState([])
  // const items = data?.searchMovieByPrefix || [];
  // const searchItemsByPrefix = debounce(searchItems, 0);
  // resetIdCounter();
  // // if (called && loading) { 
  // //   return <p>IT"S LOADING ALREADY CHILL</p>
  // // }
  // const {
  //   isOpen,
  //   getToggleButtonProps,
  //   getLabelProps,
  //   inputValue,
  //   getMenuProps,
  //   getInputProps,
  //   getComboboxProps,
  //   highlightedIndex,
  //   getItemProps,
  //   selectedItem,
  // } = useCombobox({
  //   items, 
  //   onInputValueChange() {
  //     searchItemsByPrefix({
  //         variables: { 
  //         totalResult: 10,
  //         indexName: "movies",
  //         query: inputValue 
  //       }
  //     })      
  //   },
  //   onSelectedItemChange({ selectedItem: MovieType }) { 
  //     router.push({ pathname: `/movie/${selectedItem?.movieId}`});
  //   },
  //   itemToString: (items: MovieType | null) => items?.title || '',
  // })

  return (
    <div className="relative h-screen  bg-gradient-to-b from-gray-900/10 
     to-[#010511]">
      <Head>
        <title>Netflix</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Header/>
      <main className='relative pl-5 pb-24 lg:space-y-24 lg:pl-16 top-[7rem]'>
        {/* Insert banner here for top trending movies  */}
        <div className="text-center items-center ">
          <div className='space-y-4'>
            <h1 className='font-semibold text-5xl '>Search Netflix</h1>
            <p>Serving over a billion users worldwide</p>
          </div>
          <div className='mt-[4rem] mb-4 rounded-md shadow-md justify-center 
            flex items-center  text-gray-400 focus-within:text-gray-500 '>
            <div className='relative bottom-3'>
              <SearchIcon className='align-center pl-2 absolute h-6 w-6 sm:inline cursor-pointer'/>
            </div>
            <input
              className='text-black pl-8 pr-2 py-3  md rounded-md outline-none lg:w-[20%] md:w-[40%] transition ease-in-out'
              type="text" 
              placeholder='Search..' 
              onChange={e => {
                setSearch(e.target.value);
                SearchInUse(!searchInUse)
              }}
            />
          </div>
     
          <div>
            <h3 className='font-semibold text-xl '>{related_words ? ('Related') : ('')}</h3>
            <p className='text-lg space-y-1 justify-center font-normal
              capitalize flex text-justify space-x-5 items-baseline text-red-600'>{related_words}</p>
          </div>
          <div>
          </div>

          <article className='mt-10'>
            <h1 className='flex justify-center space-y-3 font-semibold text-xl'>Available filters:</h1>
            
            <FilterBy genres={data?.searchMovie?.genres!}/>
          </article>
        </div>
        <FilterMenu 
          sort_by_popular={sort_by_most_popular}
          sort_by_least_popular={sort_by_least_popular}
          set_total_result={setTotalResult}
        />
        <section className="mt-1">
          <Col title={searchInUse ?  ('Trending Right Now') 
            : (`Search Results for: ${Search}`)
          } movies={data?.searchMovie?.movieList!}/> 
        </section>
        { showModel && <Model/>}
      </main>
    </div>

  )
}

export default Search
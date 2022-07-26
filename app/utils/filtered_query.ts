import { useQuery, gql } from '@apollo/client';
import {MovieType} from '../typings'
import client from "../apollo-client";


export const FILTERED_QUERY = gql `
    query FilterBy($filter: FilterQuery!) {
        filterBy(filter: $filter) {
            movieId
            title
            year
            awards
            countries
            genres
            homepage
            languages
            overview
            poster
            rated
            releaseDate
            runtime
            status
            videoFile
            keywords
        }
    }
`;
export interface FilteredSearchResult { 
    filterBy: MovieType[]
}
export interface FilteredSearchVariables { 
    termName: string | null,
    termValue: string | null ,
    totalResult: number | null ,
    indexName: string
}


export const FilteredSearch = { 
    crime_fiction: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Crime",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    action_movies: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Action",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    horror_movies: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Horror",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    romantic_movies: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Romance",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    documentaries_movies: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Documentary",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    comedy_movies: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Comedy",
          totalResult: 20,
          indexName: "movies"
        } }
    }),
    science_fiction: client.query<FilteredSearchResult, {filter: FilteredSearchVariables}>({ 
        query: FILTERED_QUERY,
        variables: {filter: { 
          termName: "genres.keyword",
          termValue: "Science Fiction",
          totalResult: 20,
          indexName: "movies"
        } }
    })

}

import { useQuery, gql } from '@apollo/client';
import {Genre, MovieType} from '../typings'
import client from "../apollo-client";
import { SearchResourcesSimpleCriterionKey } from 'aws-sdk/clients/macie2';

// ----------------------------- Search with filters on -------------------------//
export const SEARCH_MOVIE = gql `
    query SearchMovie($input: SearchTextInput!) {
        searchMovie(input: $input) {

            movieList {
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
            genres {
                docCount
                key
            }
        }
    }
`;

export interface AggregatedQuery { 
    genres: Genre[],
    movieList: MovieType[] 
}


export interface SearchInputs { 
    query: string,
    totalResult: number | null,
    indexName: string | null,
    sortBy: string | null, 
    order: string | null,
    aggField: string | null,
    aggSize: number | null,
    filterBy: string | null,
    filterValue: string | null
}

export interface SearchTextInput { 
    input: SearchInputs
}

// ------------------------------------------ SEARCH ALL DATABASE ------------------------// 
export const SEARCH_ALL = gql `
    query SearchApi($indexName: String!, $totalResult: Int) {
        searchApi(indexName: $indexName, totalResult: $totalResult) {
            movieId
            title
            year
            awards
            countries
            genres
            homepage
            keywords
            languages
            mediaType
            movieCasts
            movieCompany
            movieDirector
            movieWriter
            overview
            poster
            rated
            releaseDate
            runtime
            status
            videoFile
        }
    }
`;
export interface SearchAllResult { 
    searchApi: MovieType[]
}
export interface SearchAllVariables { 
    indexName: string, 
    totalResult: number
}

export async function search_all(indexName: string, totalResult: number) { 
    await client.query<SearchAllResult, SearchAllVariables>({
    query: SEARCH_ALL,
    variables: { 
        indexName,
        totalResult
    } 
  })
}
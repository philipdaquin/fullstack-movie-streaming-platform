import { useQuery, gql } from '@apollo/client';
import {MovieType} from '../typings'
import client from "../apollo-client";

export const SORTED_QUERY = gql `
    query SortMoviesAccordingly($input: SortAllMovies!) {
        sortMoviesAccordingly(input: $input) {
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
        }
    }
`;

export interface SortedSearchResult { 
    sortMoviesAccordingly: MovieType[]
}

export interface SortedSearchVariables {
    termName: string | null, 
    order: string | null,
    totalResult: number | null,
    indexName: string
}

export const ServerRequest = { 
    top_trending: client.query<SortedSearchResult, {input: SortedSearchVariables}>({
        query: SORTED_QUERY,
        variables: {input: { 
            termName: "rating.popularity",
            order: "asc",
            totalResult: 20,
            indexName: "movies"
        }}

    }),
    top_rated: client.query<SortedSearchResult, {input: SortedSearchVariables}>({
        query: SORTED_QUERY,
        variables: {input: { 
            termName: "rating.metascore",
            order: "desc",
            totalResult: 20,
            indexName: "movies"
        }}

    })

}
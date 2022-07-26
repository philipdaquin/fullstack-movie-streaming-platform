import { atom } from "recoil";
import { Genre, Movie, MovieType } from "../typings";
import {DocumentData}  from 'firebase/firestore'

export const modalState = atom({
    key: 'modalState',
    default: false,
  })
  
export const movieState = atom<MovieType | DocumentData | null>({
    key: 'movieState',
    default: null,
})



// export const genreModalState = atom({
//   key: 'genreState',
//   default: false,
// })

// export const genreState = atom<Genre | null>({
//   key: 'genreState',
//   default: null

// })
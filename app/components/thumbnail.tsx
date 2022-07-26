import React, { useState } from 'react'
import { thumbnail_w_500 } from '../global_dependencies/constants'
import { Movie, MovieType } from '../typings'
import Image from 'next/image'
import { modalState, movieState } from '../atoms/model'
import { useRecoilState } from 'recoil'
import { DocumentData } from 'firebase/firestore'
import { MoonIcon } from '@heroicons/react/solid'

interface ThumbnailProps { 
    // movie: Movie | DocumentData
    movie: MovieType | DocumentData
}
function ThumbNail({movie}: ThumbnailProps) {
    const [showModal, setShowModal] = useRecoilState(modalState);
    const [currentMovieSelected, showSelectedMovie] = useRecoilState(movieState);
    // console.log(movie.backdrop_path || movie.poster_path)
    console.log("THE MOVIE POSTER {}", movie.poster)
    // console.log(` The image url${thumbnail_w_500}${movie.backdrop_path || movie.poster_path}`);
    const poster = () => {
        if (movie.poster?.length !== 0) { 
            return movie.poster
        } else { 
           return movie.videoFile
        }
    }
    
    return (
        <div
            onClick={() => { 
                showSelectedMovie(movie);
                setShowModal(true);
            }}
            className="relative h-28 min-w-[180px] cursor-pointer transition duration-200 ease-out
            md:h-36 md:min-w-[260px] md:hover:scale-105
        ">
            <Image
                // src={`https://image.tmdb.org/t/p/w500${movie.backdrop_path || movie.poster_path}`}
                src={`${thumbnail_w_500}${poster()}` }
                className="rounded-sm object-cover md:rounded"
                layout="fill"
            />
        </div>
  )
}

export default ThumbNail
import React, { useState } from 'react'
import { thumbnail_w_500 } from '../global_dependencies/constants'
import { Movie, MovieType, MovieRating} from '../typings'
import Image from 'next/image'
import { modalState, movieState } from '../atoms/model'
import { useRecoilState } from 'recoil'
import { DocumentData } from 'firebase/firestore'
import { MoonIcon } from '@heroicons/react/solid'

interface SearchThumbnailProps { 
    // movie: Movie | DocumentData
    movie: MovieType | DocumentData
}
function SearchThumbNail({movie}: SearchThumbnailProps) {
    const [showModal, setShowModal] = useRecoilState(modalState);
    const [currentMovieSelected, showSelectedMovie] = useRecoilState(movieState);
    const poster = () => {
        if (movie.poster?.length !== 0) { 
            return movie.poster!
        } else { 
           return movie.videoFile
        }
    }
    console.log(movie.poster)
    return (
        <div onClick={() => { showSelectedMovie(movie); setShowModal(true);}}
            className="relative h-20 w-30 min-w-[180px] 
                min-h-[250px]   
                cursor-pointer transition duration-200 ease-out
                md:h-36 md:min-w-[200px] md:min-h-[220px] md:hover:scale-105 
                sm:min-h-[180px] sm:min-w-[200px]">
            <Image
                src={`${thumbnail_w_500}${poster()}` }
                className="rounded-sm object-cover md:rounded"
                layout="fill"
            />
            <article>
                <h3 className='bg-gray-400 
                    bg-opacity-30 
                    rounded-sm 
                    absolute 
                    top-3 left-1 text-white p-2 font-semibold'>{movie.title}</h3>

            </article>
        
        </div>
  )
}

export default SearchThumbNail
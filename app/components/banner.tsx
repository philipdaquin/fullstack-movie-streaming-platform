import React, { useEffect, useState } from 'react'
import Image from 'next/image'
import { Movie } from '../typings'
import { baseUrl } from '../global_dependencies/constants';
import { FaPlay} from 'react-icons/fa' 
import { InformationCircleIcon } from '@heroicons/react/solid';
import {modalState, movieState} from '../atoms/model'
import { useRecoilState } from 'recoil';

interface BannerProps { 
  netflix_original: Movie[],
}
function Banner({ netflix_original }: BannerProps) {

  

  const [movie, setMovie] = useState<Movie | null>(null);
  const [showModal, setShowModal] = useRecoilState(modalState);
  const [currentMovieSelected, showSelectedMovie] = useRecoilState(movieState);

  useEffect(() => { 
    setMovie(
      netflix_original[Math.floor(Math.random() * netflix_original.length)]
    )
  }, [netflix_original])

  return (
    <div className='
      flex flex-col space-y-2 py-16
      md:space-y-4 
      lg:h-[65vh] lg:justify-end lg:pb-12
    '>
      <div className='absolute top-0 -z-10 left-0 h-[95vh]  w-screen '>
        <Image
            layout="fill"
            src={`${baseUrl}${movie?.backdrop_path || movie?.poster_path}`}
            objectFit="cover"
          /> 
      </div>
      <h1 className='text-2xl lg:text-7xl font-bold'>{movie?.title || movie?.name || movie?.original_name}</h1>
      <p className='text-shadow-md
          max-w-xs text-xs 
          md:max-w-lg md:text-lg 
          lg:max-w-2xl lg:text-1.4xl text-gray-200'
      >{movie?.overview}</p>

      <div className='flex space-x-3'>
        <button className='banner__button text-black bg-white'>
          <FaPlay className='h-4 w-4 text-black md:h-7 md:w-7 '/> 
            Play
        </button>
      {/* When users click a video, a pop up would show up 
        which would show the movie trailer
      */}
        <button 
          onClick={() => { 
            setShowModal(true);            
            showSelectedMovie(movie); }}
          className='banner__button bg-[gray]/70'
        >
            More Info
          <InformationCircleIcon className='h-5 w-5 md:w-8'/>
        </button>
      </div>      
    </div>
  )
}

export default Banner
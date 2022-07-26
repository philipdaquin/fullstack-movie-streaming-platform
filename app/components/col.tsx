import { ChevronRightIcon } from '@heroicons/react/outline'
import {ChevronLeftIcon} from '@heroicons/react/outline'
import { DocumentData } from 'firebase/firestore'
import React, { useRef, useState } from 'react'
import { Movie, MovieType } from '../typings'
import SearchThumbNail from './search_results_thumbnail'

interface ColProps { 
    title: string,

    // Firebase: movie: Movie | DocumentData []
    // movies: Movie[] | DocumentData[] 
    movies: MovieType[] 

}
function Col({title, movies}: ColProps) {
    
    const rowRef = useRef<HTMLDivElement>(null);
    const [isMoved, setIsMoved] = useState(false);

    const handleClick = (e: string) => { 
        setIsMoved(true);

        if (rowRef.current) {
            const { scrollLeft, clientWidth} = rowRef.current;
            const scrollTo = e === "left" ? scrollLeft - clientWidth 
            : scrollLeft + clientWidth

            rowRef.current.scrollTo({left: scrollTo, behavior: 'smooth'})
        }
        console.log(rowRef.current!.scrollLeft, rowRef.current!.clientWidth)

    }
    return (
    <section>
        <div className='h-40 space-y-0.5 md:space-y-2'>
            <h2 className="text-center cursor-pointer 
                text-sm font-semibold 
                text-[#e5e5e5] 
                transition duration-200 
                hover:text-white md:text-2xl mb-5 border-1">{title}</h2>

            <div className='group relative md:-ml-2  md:mr-10'>
                {/* <ChevronLeftIcon className={`absolute top-0 
                    bottom-0 left-2 
                    m-auto z-40 h-9 w-9
                    cursor-pointer opacity-0 
                    transition hover:scale-125 
                    group-hover:opacity-100 ${!isMoved && "hidden"}`}
                    onClick={() => handleClick("left")}
                /> */}

                <div ref={rowRef} className="grid grid-flow-row
                    grid-cols-6  grid-rows-4 gap-5
                    space-x-2 space-y-2
                    md:space-x-2.5 md:p-2 scrollbar-hide "
                >
                    {movies?.filter((movie) => movie.poster).map((movie) => (
                        <SearchThumbNail key={movie.movieId} movie={movie}/>
                    ))}
                </div>
{/* 
                <ChevronRightIcon className='absolute top-0 
                    bottom-0 right-2 
                    m-auto z-40 h-9 w-9
                    cursor-pointer opacity-0 
                    transition hover:scale-125 
                    group-hover:opacity-100'
                    onClick={() => handleClick("right")}
                /> */}
            </div>
        </div>
    </section>
  )
}

export default Col
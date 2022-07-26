import { ChevronRightIcon } from '@heroicons/react/outline'
import {ChevronLeftIcon} from '@heroicons/react/outline'
import { DocumentData } from 'firebase/firestore'
import React, { useRef, useState } from 'react'
import { Movie, MovieType } from '../typings'
import ThumbNail from './thumbnail'

interface RowProps { 
    title: string,

    // Firebase: movie: Movie | DocumentData []
    // movies: Movie[] | DocumentData[] 
    movies: MovieType[] | DocumentData[]

}
function Row({title, movies}: RowProps) {
    
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
            <h2 className="w-56 cursor-pointer 
                text-sm font-semibold 
                text-[#e5e5e5] 
                transition duration-200 
                hover:text-white md:text-2xl">{title}</h2>
            <div className='group relative md:-ml-2'>
                <ChevronLeftIcon className={`absolute top-0 
                    bottom-0 left-2 
                    m-auto z-40 h-9 w-9
                    cursor-pointer opacity-0 
                    transition hover:scale-125 
                    group-hover:opacity-100 ${!isMoved && "hidden"}`}
                    onClick={() => handleClick("left")}
                />

                <div ref={rowRef} className="flex items-center 
                    space-x-0.5 overflow-x-scroll 
                    md:space-x-2.5 md:p-2 scrollbar-hide"
                >
                    {movies?.map((movie) => (
                        <ThumbNail key={movie.movieId} movie={movie}/>
                    ))}
                </div>

                <ChevronRightIcon className='absolute top-0 
                    bottom-0 right-2 
                    m-auto z-40 h-9 w-9
                    cursor-pointer opacity-0 
                    transition hover:scale-125 
                    group-hover:opacity-100'
                    onClick={() => handleClick("right")}
                />
            </div>
        </div>
    </section>
  )
}

export default Row
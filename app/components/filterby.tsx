import React from 'react'
import { Genre } from '../typings'
import GenreFilter from '../components/genre_filter'

interface Props { 
    genres: Genre[],
}
function FilterBy({genres}: Props) {
  return (
    <div className='flex space-x-2 justify-center p-2 gap-8'>
        {
            genres?.map((genre) => ( 
                <GenreFilter genre={genre} key={genre.key}/>
            ))
        }
    </div>
  )
}

export default FilterBy
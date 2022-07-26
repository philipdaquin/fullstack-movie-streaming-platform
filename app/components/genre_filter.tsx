import React from 'react'
import { Genre } from '../typings'
import { useRecoilState } from 'recoil'

interface Props { 
    genre: Genre 
}

function GenreFilter({genre}: Props) {
  return (
    <div
        className='shadow-md  flex items-center justify-center span-col-4 p-2 bg-gray-500 rounded-md 
        hover:bg-gray-300 hover:text-gray-800 cursor-pointer
    '>
        <div
            className='flex space-x-5 items-center'>
            <h1 className='font-medium'>{genre.key}</h1>
            <h2 className='font-normal text-sm'>{genre.docCount}</h2>
        </div>
    </div>
  )
}

export default GenreFilter
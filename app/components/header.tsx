import React, { useEffect, useState } from 'react'
import {BellIcon, SearchIcon} from '@heroicons/react/solid'
import Link from 'next/link'
import useAuth from '../hooks/useAuth';
import BasicMenu from './menu';

import { gql, useQuery } from '@apollo/client';
import { Movie, MovieType } from '../typings';
import Search from '../pages/search';
// import Movie from '../pages/content_creator'


function Header() {
  const [isScrolled, setScrolled] = useState(false);
  const { user,  log_out} = useAuth();

  useEffect(() => { 
    const handleScroll = () => { 
      if (window.scrollY > 0 ) { 
        setScrolled(true);
      } else { 
        setScrolled(false);
      }
    }
    window.addEventListener('scroll', handleScroll);
    
    return () => { 
      window.removeEventListener('scroll', handleScroll)
    }
  }, [])

  const [searchButtonPressed, setSearchButtonPressed] = useState(false)

  return (
    <>
      <header className={`${isScrolled && 'bg-[#141414]/70 ease-in-out'}`}>
        <div className='flex items-center space-x-2 md:space-x-10'>
          <Link href="/" className='cursor-pointer'>
            <img 
              src="https://upload.wikimedia.org/wikipedia/commons/0/08/Netflix_2015_logo.svg" 
              alt="" 
              width={100}
              height={100}
              className="cursor-pointer object-contain"
            />
          </Link>
          <BasicMenu />
          <nav className="">
            <ul className='hidden space-x-4 md:flex'>
              <li className='headerLink cursor-default font-semibold text-white hover:text-white'><a href="/">Home</a></li>
              <li className='headerLink'><a href="">TV Shows</a></li>
              <li className='headerLink'><a href="">Movies</a></li>
              <li className='headerLink'><a href="">News & Popular</a></li>
              <li className='headerLink'><a href="">My List</a></li>
              <li className='headerLink'>
                <a href="/content_creator" 
                className='p-2 bg-white text-black rounded-md shadow-md
                  font-semibold cursor-pointer hover:text-gray-600'>
                  For Creators
                  </a>
                </li>
            </ul>
          </nav>
        </div>

        <div className='flex items-center space-x-4 text-sm font-light'>
          {/* <Search/> */}
          <button onClick={e => setSearchButtonPressed(!searchButtonPressed)} className="transition ease-in-out duration-500">
            {/* {
              searchButtonPressed?  ( */}
                <div className='flex space-x-3'>
                  <Link href="/search">
                    <SearchIcon className='h-6 w-6 sm:inline cursor-pointer' />
                  </Link>
                </div>
                {/* ) : (
                <SearchIcon className='h-6 w-6 sm:inline cursor-pointer'/>
                )  
            } */}
          </button>
          <p className="hidden lg:inline">Kids</p>
          <BellIcon className='h-6 w-6 cursor-pointer'/>
          <Link href="/account">
            <img 
              // onClick={log_out}
              src="https://rb.gy/g1pwyx" 
              alt="" 
              className='cursor-pointer rounded'
            />
          </Link>
        </div>

       

      </header>
    </>
  )
}

export default Header
import { Button, CircularProgress } from '@mui/material'
import React, { useState } from 'react'
import Header from '../components/header'
import { useS3Upload } from "next-s3-upload";
import { XIcon } from '@heroicons/react/solid';
import { ArrowCircleUpIcon, PlusIcon } from '@heroicons/react/outline';
import { SubmitHandler } from 'react-hook-form';
import { gql , useMutation} from '@apollo/client';
import {MovieType, NewMovieType, MovieRating, BusinessData} from '../typings' 

const ADD_NEW_MOVIE_DATA = gql `
    mutation CreateMovie($newMovie: NewMovieInput!) {
      createMovie(newMovie: $newMovie) {
        movieId
        title
        year
        awards
        business {
          budget
          revenue
        }
        countries
        genres
        homepage
        keywords
        languages
        mediaType
        movieCasts
        movieCompany
        movieDirector
        movieWriter
        overview
        poster
        rated
        rating {
          imdbId
          metascore
          popularity
          voteCount
          voteAverage
        }
        releaseDate
        runtime
        status
        videoFile
      }
    }
`;

interface Props { 
    title: string,
    backdrop_url?: string
}


function ContentCreator() {
  let [uploading, setUploading] = useState(false)  
  const [filename, setFilename] = useState<File | null>();
  let { FileInput, openFileDialog, uploadToS3, files } = useS3Upload();
  let [video, setVideo] = useState<string>();
  let [key, setKey] = useState<string>();
  let [bucket, setBucket] = useState<string>();

  const handleSetFile = async (file: File) => {
    setUploading(true);     
    //  Upload the file to s3 
    const { url, key, bucket } = await uploadToS3(file);
    //  Update the state of the url, key and bucket which all will be stored in our 
    //  databse
    setFilename(file);
    setVideo(url[0]);
    setKey(key);
    setBucket(bucket);
    //  keep track of the file progress
    console.log(url);
    setUploading(false);
  }
  const [title, setTitle] = useState('');
  const [year, setAdult] = useState<number>();
  const [awards, setAwards] = useState<string[]>();
  const [countries, setCountries] = useState<string[]>();
  const [genres, setGenres] = useState<string[]>();
  const [homepage, setHomepage] = useState<string>();
  const [keywords, setKeywords] = useState<string[]>();
  const [languages, setLanguages] = useState<string[]>();
  const [media_type, setMediaType] = useState<string>();
  const [movie_casts, setMovieCasts] = useState<string[]>();
  const [movie_company, setMovieCompany] = useState<string[]>();
  const [movie_director, setMovieDirector] = useState<string[]>();
  const [movie_writer, setMovieWriter] = useState<string[]>();
  const [overview, setOverview] = useState('');
  const [poster, setPoster] = useState('PLANNED');
  const [rated, setRated] = useState('');
  const [release_date, setReleaseDate] = useState('');
  const [status, setStatus] = useState('Rumoured');
  const [video_file, setVideoFile] = useState<string>();
  const [runtime, setRuntime] = useState<number>()
  //  Custom types 
  const [movieRating, setMovieRating] = useState<MovieRating>();
  const [businessData, setBusinessDate] = useState<BusinessData>()


  let [createMovie, {error, data}] = useMutation<{createMovie: MovieType}, {newMovie: NewMovieType}>(
    ADD_NEW_MOVIE_DATA, {
        variables: { newMovie: {
            title, 
            year,
            awards,
            business: businessData,
            countries,
            genres,
            homepage,
            keywords,
            languages,
            mediaType: media_type,
            movieCasts: movie_casts,
            movieCompany: movie_company,
            movieDirector: movie_director,
            movieWriter: movie_writer,
            overview,
            poster,
            rated,
            rating: movieRating,
            releaseDate: release_date,
            runtime,
            status,
            videoFile: video_file,
        }}
    });
  const onSubmit: SubmitHandler<NewMovieType> = () => { 
        createMovie();
}

  return (
    <>
      <Header />
      {/* <button>Text</button> */}
      <section className='flex flex-col h-screen bg-gray-50'>
        <section>
          <article className="relative">
            <div className="p-60 md:p-[5rem] md:left-4">
          <div className='flex items-center justify-center'>
            {data ? <p className='text-black text-lg font-medium relative top-6 right-2'> Returned Data: Success ✅✅✅</p> : null}
            {error ? <p> Something went wrong! {error.message}</p> : null}
          </div>
              <h1 className='text-left p-6 relative left-2 font-medium text-[2.1rem] text-black'>Upload a movie</h1>
              {/* Right Column */}
              <div className='text-black float-right grid px-2 min-w-[55%] relative right-19'>
                <ul>
                  <h1 className='font-medium text-2xl relative bottom-3'>Add the details</h1>
                  <li className='mb-[1rem]'>
                    <div className='flex items-center align-bottom '>
                      <h2 className='font-regular text-xl pb-2'>Movie Title</h2>
                      <p className='text-xs inline-flex absolute right-1 text-gray-600/70'>*Required {title.length} / 100</p>
                    </div>
                      <input type="text" className='border 
                        shadow-sm border-solid border-gray-300 w-full  bg-transparent 
                        px-3 rounded m-0 block 
                        outline-none'
                        onChange={e => setTitle(e.target.value)}  
                      />
                  </li>
                  <li className='mb-[1rem]'>
                      <h2 className='font-regular text-xl pb-4'>Description</h2>
                      <textarea 
                        onChange={e => setOverview(e.target.value)}
                        rows={3}
                        className=" form-control block w-full px-3 py-1.5 text-base shadow-sm
                          font-normal text-gray-700 bg-white bg-clip-padding border 
                          border-solid border-gray-300 rounded transition ease-in-out m-0 outline-none
                      "></textarea>
                  </li>
                  <li className='mb-[1rem]'>
                    <h2 className='font-regular text-xl pb-2'>People In this People</h2>
                    <input 
                      // todo!()
                      type="text" 
                      className='border 
                        shadow-sm border-solid border-gray-300 w-full  bg-transparent 
                        px-3 rounded m-0 block 
                        outline-none'/>
                  </li>
                  <li className='flex gap-10 relative top-3 mb-[1rem]'>
                    <div className='items-center flex space-x-6'>
                      <div className='text-left'>
                        <h2>Movie Cover:</h2>
                        <p className='font-regular text-xs'>File name: {}</p>
                      </div>
                      <button 
                      onClick={openFileDialog}
                      className='shadow-sm relative bottom-1 items-center flex justify-center 
                        bg-gray-500/20 w-[6rem] h-[2rem] rounded-lg'>
                        <PlusIcon className='h-[1.5rem] w-[1.5rem] stroke-[2px] stroke-gray-500/90 decoration-1'/>
                      </button>
                    </div>
                    <div className='items-center flex space-x-7 relative bottom-1'>
                      <h2>Homepage/ URL:</h2>
                      <input 
                        onChange={e => setHomepage(e.target.value)}
                        type="text" 
                        className='border 
                          shadow-sm border-solid border-gray-300 w-full bg-transparent 
                          rounded m-0 block 
                          outline-none'/>
                    </div>
                  </li>
                  <li className='mb-[1rem]'>
                    <h2 className='font-regular text-xl pb-2'>Genres</h2>
                    <input 
                      onChange={e => setGenres([e.target.value])}
                      type="text" 
                      className='border 
                        shadow-sm border-solid border-gray-300 w-full  bg-transparent 
                        px-3 rounded m-0 block 
                        outline-none'/>
                  </li>
                  <li className='mb-[1rem]'>
                    <div className='items-center flex space-x-6'>
                      <h2 className='font-regular text-xl pb-3'>Production Company</h2>
                      <p className='text-xs inline-flex text-gray-600/70'>*Optional</p>
                      <input 
                        onChange={e => setCompany([e.target.value])}
                        type="text" 
                        className='border 
                          shadow-sm border-solid border-gray-300 w-full  bg-transparent 
                           rounded m-0 block 
                          outline-none'/>
                    </div>
                    <div className='items-center'>
                      <h2>,</h2>
                      <input 
                       
                        type="text" 
                        className='border 
                        shadow-sm border-solid border-gray-300 w-full bg-transparent 
                        rounded m-0 block 
                        outline-none'/>
                    </div>

                  </li>
                </ul>
                <div className='flex justify-end'>
                  <button 
                      onClick={() => onSubmit({
                        title, 
                        
                    })} 
                  className='font-medium shadow-sm rounded-md bg-gray-500/20 w-[7rem] h-[2rem]'>Upload</button>
                </div>
              </div>

              {/* Left Column */}
              <div className="relative h-[20%] w-[35%] bg-gray-500/20 shadow-lg
                    md:h-[40rem] md:left-[2rem]">
                      {/* Insert Media Here */}
                      <div className='flex flex-col justify-center min-h-full relative bottom-6'>
                        <ul className='text-center'>
                            <h1 className="font-semibold text-lg text-gray-500/80">Upload a video here</h1>
                            <p className='font-regular text-xs text-gray-500/80'>MP4, MOV, WEBm are allowed</p>
                        </ul>
                        <div className='flex flex-col items-center justify-center pt-[5rem]'>
                            {uploading ? (
                              <>
                                <CircularProgress 
                                      color='primary' 
                                      variant="determinate" 
                                      value={100}
                                      thickness={3}
                                      sx={{
                                        color: (theme) => theme.palette.grey[theme.palette.mode === 'light' ? 600 : 800],
                                        position: 'absolute'
                                        }}
                                      size={100} />
                                <XIcon className="h-8 w-8 absolute" />
                  
                                 { 
                                    files.map((file, index) => ( 
                                    <CircularProgress 
                                      variant="determinate" 
                                      value={file.progress}
                                      thickness={3} 
                                      sx={{ color: (theme) => (theme.palette.mode === 'light' ? '#fff' : '#fff') ,position: 'absolute'}}
                                      size={100} />
                                    ))
                                  }

                              </>
                            ): (
                              <ArrowCircleUpIcon  className='h-[8rem] w-[8rem] absolute stroke-[0.75px] stroke-gray-500/80 decoration-1'/>
                            ) 
                            } 
                        </div>
                        <div className='relative flex justify-center top-[4.5rem]'>
                          <ul className='items-center text-center'>
                            <button
                              onClick={openFileDialog} 
                              className='bg-[#ABABAB] w-[8rem] h-[2rem] rounded-lg'>Browse</button>
                            <li className='relative top-4 font-semibold text-sm text-gray-500/80'>
                                <p>
                                  1280 x 720 resolution or higher
                                </p>
                                <p>
                                  Up to 60s
                                </p>
                                <FileInput onChange={handleSetFile} />
                                <ul className='mt-2'>
                                  <li><p>File Name: {filename?.name}</p></li>
                                </ul>
                            </li>
                          </ul>
                        </div>
                      </div>
                  
              </div>
            </div>
          </article>
        </section>
      </section>
    </>
  )
}

export default ContentCreator

function setCompany(arg0: string[]): void {
  throw new Error('Function not implemented.');
}

import React, { useEffect, useState } from 'react'
import MuiModal from '@mui/material/Modal'
import { useRecoilState, useRecoilValue } from 'recoil';
import { modalState, movieState } from '../atoms/model';
import Box from '@mui/material/Box';
import { PlusIcon, ThumbUpIcon, XIcon, CheckIcon } from '@heroicons/react/solid';
import { ThumbUpIcon  as ThumbUpIconOutline} from '@heroicons/react/outline';

import { Genre, MovieType, MovieRating, BusinessData } from '../typings';
import { movieApi, trailerMovie } from '../global_dependencies/constants';
import {Element } from '../typings';
import ReactPlayer from 'react-player/lazy';
import { FaPlay } from 'react-icons/fa';
import VolumeUpIcon from '@heroicons/react/outline/VolumeUpIcon';
import VolumeOffIcon from '@heroicons/react/outline/VolumeOffIcon';
import { totalmem } from 'os';
import use_list from '../hooks/use_list';
import { collection, deleteDoc, doc, DocumentData, onSnapshot, setDoc } from 'firebase/firestore';
import { firestore as db } from '../firebase';
import useAuth from '../hooks/useAuth';
import toast, { Toaster, useToasterStore } from 'react-hot-toast';


function Model() {
  const { user } = useAuth();
  const [showModal, setShowModal] = useRecoilState(modalState);
  const [currentMovieSelected, showSelectedMovie] = useRecoilState(movieState);
  const [trailer, setTrailer] = useState('');
  const [genres, setGenres] = useState<Genre[]>([]);
  const [muted, setMuted] = useState(true);
  const [play, setPlay] = useState(false);
  const [liked, setLiked] = useState(false);
  const [added, setAdded] = useState(false);

  const [movielist, setMovielist] = useState<DocumentData[] | MovieType[]>([]);

  console.log("Show Movie $1",currentMovieSelected);

  useEffect(() =>  {
      if (!currentMovieSelected)  { return }
      
      async function fetchMovie() { 
          const data = await fetch(`${movieApi}${currentMovieSelected?.mediaType === 'tv' ? 'tv' : 'movie'
              }/${currentMovieSelected?.movieId}?api_key=${process.env.NEXT_PUBLIC_BASE_URL
              }&language=en-US&append_to_response=videos`)
            .then((resp) => resp.json())
            .catch((e) => e.message);

          if (data?.videos) { 
            const index = data.videos.results.findIndex((element: Element) => 
              element.type === 'Trailer' );
              setTrailer(data.videos?.results[index]?.key) 
            }
          if (data?.genres) { 
            setGenres(data.genres);
          }
      };

      fetchMovie()
  }, [currentMovieSelected])
        
  //  Update the user movie list
  useEffect(() => { 
    if (user) { 
      return onSnapshot(
        collection(db, "Customer", user?.uid, "List1"), (snapshot) => setMovielist(snapshot.docs) 
      )
    }
  }, [db, currentMovieSelected?.movieId])

  useEffect(() => { 
    setAdded(movielist.findIndex((res) => res.data().id === currentMovieSelected?.movieId) !== -1)},
     [movielist])

  const handleClose = () => { 
    setShowModal(false);
    showSelectedMovie(null);
    setPlay(false);
  }

  const handleLikeButton = () => { 
    if (!liked) { 
      toast(`You liked ${currentMovieSelected?.title || currentMovieSelected?.languages} 👍`, {duration: 8000})
    } else { 
      toast(`You disliked ${currentMovieSelected?.title || currentMovieSelected?.title} 👎`, {duration: 8000})
    }
    setLiked(!liked);
  }

  const handleMuteButton = () => { 
    if (!muted) { 
      toast(`🔇`, {duration: 8000})
    } else { 
      toast(`🔊`, {duration: 8000})
    }
    setMuted(!muted)
  }

  const handleAddedToListButton = async () => { 
    if (added) { 
      await deleteDoc(doc(db, 'Customers', user!.uid, 'List1', 
      currentMovieSelected?.movieId.toString()! ));


      toast(`${currentMovieSelected?.title || currentMovieSelected?.homepage} 
      has been removed from the list.`, { duration: 8000 })
    } else {
    
      
      await setDoc(doc(db, "Customers", user!.uid, 'List1', currentMovieSelected?.movieId.toString()!), 
        {...currentMovieSelected}) 
      toast(`${currentMovieSelected?.title || currentMovieSelected?.homepage} 
      has been added from the list.`, { duration: 8000})
    }
  }

  console.log(" This is the genre of movie  $1", movielist);

  return (
    <MuiModal
        className="fixes !top-7 left-0 right-0 
          z-50 mx-auto w-full max-w-5xl 
          overflow-hidden overflow-y-scroll 
          rounded-md scrollbar-hide
        "
        open={showModal}
        onClose={handleClose}>
        <>
          <Toaster position='bottom-center'/>

            <button 
                onClick={handleClose} 
                className="modalButton absolute 
                    right-5 top-5 !z-40 h-9 w-9
                    border-none bg-[#181818] hover:bg-[#333333]"
            >
                <XIcon className="h-6 w-6" />
            </button>
            <div className="relative pt-[56.25%]"> 
              <ReactPlayer
                url={`${trailerMovie}${trailer}`}
                width="100%"
                height="100%"
                style={{
                  position: 'absolute',
                  top: '0',
                  left: '0',
                }}
                playing={play}
                muted={muted}
              />
              <div className='absolute bottom-10 flex justify-between w-full items-center px-10'>
                {/* Play button */}
                <div className='flex space-x-2'>
                  <button 
                    onClick={() => setPlay(true)}
                    className='banner__button bg-white text-black'>
                    <FaPlay className="h-7 w-7 text-black"/>
                    Play
                  </button>

                  <button className='modalButton' onClick={handleAddedToListButton}>
                    { added? ( 
                      <CheckIcon className='h-7 w-7'/> 
                      ) : ( 
                        <PlusIcon className='h-7 w-7'/>
                      ) 
                    }
                  </button>
                  {/* Like Button */}
                  <button 
                    onClick={() => {handleLikeButton()}}
                    className='modalButton'>
                    {!liked? <ThumbUpIconOutline className='h-7 w-7'/> : <ThumbUpIcon className='h-7 w-7'/> }
                  </button>
                </div>
                
                <button onClick={() => {handleMuteButton()}} className="modalButton absolute right-9">
                  {
                    muted ? ( 
                      <VolumeOffIcon className='h-6 w-6 '/>
                     ) : (
                      <VolumeUpIcon className='h-6 w-6'/>
                     )
                  }
                </button>
              </div>
            </div>

            <section className='flex space-x-16 bg-[#181818]  rounded-b-md  p-6'>
              <article className='space-y-4 text-lg '>
                <div className="flex space-x-2 items-center text-sm">
                  {/* Ratings */}
                  <p className="font-semibold text-green-400 px-4 ">
                    {currentMovieSelected!.rating?.vote_average * 10}% Match
                  </p>
                  {/* Release Date */}
                  <p className='font-light'>{currentMovieSelected?.releaseDate}</p>
                  {/* HD LOgo */}
                  <figure className="flex justify-center 
                    border border-white/40 rounded 
                    px-1.5 text-xs">
                      HD
                  </figure>
                </div>

                {/* Description  */}
                <div className='flex flex-col gap-x-10 gap-y-4 font-regular md:flex-row  space-y-4 '>
                  {/* Main Overview of Movie */}
                  <p className='w-5/6 p-4'>{currentMovieSelected?.overview}</p>
                  {/* Right Column: Genres etc */}
                  <div className='flex flex-col space-y-3 text-sm'>
                    <div>
                      <span className='text-[gray]'>Genres: </span>
                      {genres.map((g) => g.key ).join(', ')}   
                    </div>
                    <div>
                      <span className='text-[gray]'>Original Languages: </span>
                      {currentMovieSelected?.languages}
                    </div>
                    <div>
                      <span className='text-[gray]'>Total Votes: </span>
                      {currentMovieSelected?.rating?.vote_average}
                    </div>
                  </div>
                </div>
              </article>
            </section>
        </>
    </MuiModal>
  )
}

export default Model
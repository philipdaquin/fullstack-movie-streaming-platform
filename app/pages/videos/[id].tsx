import { useRouter } from 'next/router';
import React from 'react'
import VideoPlayer from '../../components/video_player';

function VideoPage() {
  const router = useRouter();
  const {id} = router.query as { id: string };

  console.log({id});

  return (
    <div>
        <VideoPlayer id={id}/>
    </div>
  )
}

export const getServerStaticProps = async (context: { query: string }) => {
    return { 
        props: { 
            query: context.query
        }
    } 

}


export default VideoPage
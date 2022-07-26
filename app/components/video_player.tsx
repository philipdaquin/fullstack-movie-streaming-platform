import React from 'react'
import ReactPlayer from 'react-player/lazy'

interface Props {
    id: string
}

function VideoPlayer({id}: Props) {
  return (
    <div>
        <video
            src={`/api/video?videoId=${id}`}
            width="auto"
            height="auto"
            style={{
              position: 'absolute',
              top: '0',
              left: '0',
            }}
            controls
            autoPlay
            id="video-player"
        />
    </div>
  )
}

export default VideoPlayer
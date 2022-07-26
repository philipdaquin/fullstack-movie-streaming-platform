// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import busboy from 'busboy';
import fs from 'fs'

type Data = {
  name: string,
}


export const config = { 
  api:  { 
    bodyParser: false
  }
}

function upload_method(req: NextApiRequest, res: NextApiResponse) { 
  let bb = busboy({headers: req.headers});
  
  bb.on('file', (_, file, info) => { 
    let filename = info.filename;
    let filepath = `./videos/${filename}`;

    let stream = fs.createWriteStream(filepath);
    
    file.pipe(stream);
  });
  
  bb.on('close', () => {
    res.writeHead(200, { Connection: 'close'});
    res.end(`Uploaded successfully!`)  
  });
  //  Connect to readable stream 
  req.pipe(bb);
  return;
} 


const CHUNK_SIZE_IN_BYTES = 10000; 
function getVideoStream(req: NextApiRequest, res: NextApiResponse) { 
  let range = req.headers.range;

  if (!range) { 
    return res.status(400).send('Range Out of bounds')
  }

  let videoId = req.query.videoId;
  //  Send to S3 instead 
  let videoPath  = `./videos/${videoId}.mp4`;

  let videoSizeInBytes = fs.statSync(videoPath).size;
  console.log(videoSizeInBytes);

  let chunkStart = Number(range.replace(/\D/g, ""));
  let chunkEnd = Math.min(
    chunkStart + CHUNK_SIZE_IN_BYTES, videoSizeInBytes - 1
  );

  let contentLength = chunkEnd - chunkStart  + 1

  let headers = { 
    'Content-Range': `bytes ${chunkStart}-${chunkEnd}/${videoSizeInBytes}`,
    'Accept-Ranges': 'bytes',
    'Content-Length': contentLength,
    'Content-Type': 'video/mp4'
  };
  res.writeHead(206, headers);

  //  Range of chunks
  const videoStream = fs.createReadStream(videoPath, { 
    start: chunkStart,
    end: chunkEnd
  });
  console.log(chunkStart)
  //  this is what goes in the client
  videoStream.pipe(res);
}


export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {

  let method = req.method;

  if (method === "GET") return getVideoStream(req, res)
  if (method === "POST") return upload_method(req, res);

  res.status(405).json({ name: `ERROR! Method ${method} is not allowed here!`});
}

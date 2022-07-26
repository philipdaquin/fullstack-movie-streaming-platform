export interface Genre { 
    docCount: number,
    key: string
}

export interface Movie { 
    id: number,
    title: string,
    backdrop_path: string,
    media_type?: string,
    release_date?: string,
    first_air_date: string,
    genre_ids: number[],
    name: string,
    origin_country: string[],
    original_language: string,
    original_name: string,
    overview: string,
    popularity: number,
    poster_path: string,
    vote_average: number,
    vote_count: number
}

export interface BusinessData {
    bugdet: number,
    revenue: number
}

export interface MovieRating { 
    imdb_id: string,
    metascore: number, 
    popularity: number,
    vote_count: number, 
    vote_average: number
}


export interface MovieType { 
    movieId: string,
    title: string,
    year: number,
    awards: string[],
    business?: BusinessData,
    countries: string[],
    genres: string[],
    homepage: string,
    keywords: string[],
    languages: string[],
    mediaType: string,
    movieCasts: string[],
    movieCompany: string[],
    movieDirector: string[],
    movieWriter: string[],
    overview: string,
    poster: string,
    rated: string,
    rating?: MovieRating,
    releaseDate: string,
    runtime: number,
    status: string,
    videoFile: string,
}

export interface NewMovieType { 
    title: string,
    year?: number,
    awards?: string[],
    business?: BusinessData,
    countries?: string[],
    genres?: string[],
    homepage?: string,
    keywords?: string[],
    languages?: string[],
    mediaType?: string,
    movieCasts?: string[],
    movieCompany?: string[],
    movieDirector?: string[],
    movieWriter?: string[],
    overview?: string,
    poster?: string,
    rated?: string,
    rating?: MovieRating,
    releaseDate?: string,
    runtime?: number,
    status?: string,
    videoFile?: string,
}



export interface Element {
    type:
      | 'Bloopers'
      | 'Featurette'
      | 'Behind the Scenes'
      | 'Clip'
      | 'Trailer'
      | 'Teaser'
  }




  
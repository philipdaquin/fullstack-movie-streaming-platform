#![allow(dead_code)] 
use std::sync::Arc;

use actix_web::web::JsonBody;
use common_utils::QueryResult;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike, Utc};
use crate::generate_unique_id;
use futures::stream::StreamExt;
use super::movies::model::{Movie, NewMovie, BusinessData, MediaType, Status, MediaRated, MovieRating};
use lazy_static::lazy_static;
use parking_lot::Mutex;
const TMDB_URL: &str = "https://api.themoviedb.org/3";

lazy_static! { 
    static ref TMDB_API_KEY: String = std::env::var("TMDB_API_KEY").expect("Unable to get a value api key");
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct MoviesResponse<T> {
    pub page: i32,
    pub results: Option<Vec<T>>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TmdbMovie {
    pub adult: bool,
    pub backdrop_path: String,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i32,
}
///      It fetch the popular movies from tmdb
///    let tmdb_url = format!("{api}/{discover_api}/{endpoint_popular}?api_key={key}&language=en-US&with_genres=28",
pub async fn fetch_movies_externally(discover_api: String, endpoint_popular: String, language: String, included_with: String,page: i32, ) -> QueryResult<Vec<Movie>> {
    log::info!("Fetching Movies From TDb");
    let tmdb_url = format!("{api}/{discover_api}/{endpoint_popular}?api_key={key}&language={language}&{included_with}={page}",
                            api = &TMDB_URL,
                            // discover_api = discover_api.as_str(),
                            // endpoint_popular = endpoint_popular.as_str(),
                            // language = language.as_str9)
                            // included_with = included_with.as_str()
                            key = TMDB_API_KEY.as_str()
                        );
    let response = reqwest::get(&tmdb_url).await.expect("Unable to get valid request body");
    let movies: MoviesResponse<TmdbMovie> = response.json().await.expect("");
    log::info!("{:#?}", movies);

    let imdb_movies: Vec<Movie> = movies
    .results
        .unwrap_or_default()
        .iter()
        .map(|f| Movie::from(f))
        .collect::<Vec<Movie>>();
    Ok(imdb_movies)
}

impl From<&TmdbMovie> for Movie { 
    fn from(f: &TmdbMovie) -> Self {

        let release = NaiveDate::parse_from_str(&f.release_date, "%Y-%m-%d").expect("unable to parse date");
        
        Self {
            movie_id: f.id.clone() as i64,
            title: f.title.clone(),
            year: release.year(),

            awards: vec![String::new()],
            business: BusinessData::default(),
            countries: vec![String::new()],
            genres: f.genre_ids.iter().map(|f| f.to_string()).collect::<Vec<String>>(),
            homepage: String::new(),
            keywords: vec![String::new()],
            languages: vec![String::new()],
            media_type: MediaType::default().to_string(),
            movie_casts: vec![String::new()],
            movie_company: vec![String::new()],
            movie_director: vec![String::new()],
            movie_writer: vec![String::new()],

            overview: f.overview.clone(),
            poster: f.backdrop_path.clone(),
            rated: MediaRated::default().to_string(),
            rating: MovieRating::new(None, None, Some(f.popularity as f32), Some(f.vote_count.into()), Some(f.vote_average as f32)),
            release_date: release.clone(),
            runtime: 120,
            status: Status::default().to_string(),
            
            video_file: f.backdrop_path.clone(),
        }
    }
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct GenreType { 
    id: i32, 
    name: Option<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct ProductionCompany { 
    name: Option<String>, 
    id: i32, 
    logo_path: Option<String>,
    origin_country: String
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct ProductionCountry { 
    iso_3166_1: String,
    name: Option<String> 
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct SpokenLanguages { 
    iso_639_1: String,
    name: Option<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct MovieDetails { 
    #[serde(alias = "adult")]
    adult: Option<bool>,
    #[serde(alias = "id")]
    id: Option<i32>, 
    #[serde(alias = "backdrop_path")]
    backdrop_path: Option<String>,
    #[serde(alias = "belongs_to_collection")]
    belongs_to_collection: Option<Collection>,
    #[serde(alias = "budget")]
    budget: Option<i64>,
    #[serde(alias = "genres")]
    genres: Option<Vec<GenreType>>,
    #[serde(alias = "homepage")]
    homepage: Option<String>,
    #[serde(alias = "imdb_id")]
    imdb_id: Option<String>,
    #[serde(alias = "original_language")]
    original_language: Option<String>, 
    #[serde(alias = "original_language")]
    original_title: Option<String>, 

    #[serde(alias = "overview")]
    overview: Option<String>,
    #[serde(alias = "popularity")]
    popularity: Option<f64>,
    #[serde(alias = "poster_path")]
    poster_path: Option<String>,
    #[serde(alias = "production_companies")]
    production_companies: Option<Vec<ProductionCompany>>,
    #[serde(alias = "production_counties")]
    production_countries: Option<Vec<ProductionCountry>>,
    #[serde(alias = "release_date")]
    release_date: Option<String>,
    #[serde(alias = "revenue")]
    revenue: Option<i64>,
    #[serde(alias = "runtime")]
    runtime: Option<i32>,
    #[serde(alias = "spoken_languages")]
    spoken_languages:  Option<Vec<SpokenLanguages>>,
    #[serde(alias = "status")]
    status: Option<String>,
    #[serde(alias = "tagline")]
    tagline: Option<String>,
    #[serde(alias = "title")]
    title: Option<String>,
    #[serde(alias = "video")]
    video: Option<bool>, 
    #[serde(alias = "vote_average")]
    vote_average: Option<f64>, 
    #[serde(alias = "vote_count")]
    vote_count: Option<i32>

}
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Collection { 
    id: i32, 
    name: String,
    overview: String,
    poster_path: Option<String>,
    backdrop_path: String,
    parts: Vec<TmdbMovie>
}

// 1. Clean up the data
impl From<&MovieDetails> for Movie { 
    fn from(f: &MovieDetails) -> Self {
        log::info!("🧭 Convertin MovieDetails to Movie b{:#?}", f.clone());
      
        
        let default_countries = if f.production_countries.clone().is_some() { 
            if !f.production_countries.clone().unwrap().is_empty() {
                f.production_countries
                    .clone()
                    .unwrap_or_default()             
                    .iter()
                    .map(|f| f.name.clone().unwrap_or(String::new()))
                    .collect::<Vec<String>>()
            } else { 
                vec![String::new()]
            }
        } else { 
            vec![String::new()]
        };
        let default_genre = if f.genres.is_some() { 
            if !f.genres.clone().unwrap().is_empty() {
                f.genres
                    .clone()
                    .unwrap_or_default()             
                    .iter()
                    .map(|f| f.name.clone().unwrap_or(String::new()))
                    .collect::<Vec<String>>()
            } else { 
                vec![String::new()]
            }
        } else { 
            vec![String::new()]
        };
        let default_lang = if f.spoken_languages.is_some() { 
            if !f.spoken_languages.clone().unwrap().is_empty() {
                f.spoken_languages
                    .clone()
                    .unwrap_or_default()             
                    .iter()
                    .map(|f| f.name.clone().unwrap_or(String::new()))
                    .collect::<Vec<String>>()
            } else { 
                vec![String::new()]
            }
        } else { 
            vec![String::new()]
        };
        let default_company = if f.production_companies.is_some() { 
            if !f.production_companies.clone().unwrap().is_empty() {
                f.production_companies
                    .clone()
                    .unwrap_or_default()             
                    .iter()
                    .map(|f| f.name.clone().unwrap_or(String::new()))
                    .collect::<Vec<String>>()
            } else { 
                vec![String::new()]
            } 
        } else { 
            vec![String::new()]
        };


        let release = NaiveDate::parse_from_str(
            &f.release_date.clone().unwrap_or_default(), 
            "%Y-%m-%d"
        ).unwrap_or(NaiveDate::from_ymd(2015, 9, 8));
        Self {
            movie_id: f.id.clone().unwrap_or_default() as i64,
            title: f.title.clone().unwrap_or_default(),
            year: release.year(),
            awards: vec![String::new()],
            business: BusinessData::new(f.budget, f.revenue),
            countries: default_countries,
            genres: default_genre,
            homepage: f.homepage.clone().unwrap_or_default(),
            keywords: vec![String::new()],
            languages: default_lang,
            media_type:  MediaType::default().to_string(),
            //  Asynchronous request
            movie_casts: vec![String::new()],
            movie_company: default_company,
            // Asynchronous request
            movie_director: vec![String::new()],
            movie_writer: vec![String::new()],

            overview: f.overview.clone().unwrap_or_default(),
            poster: {
                if f.poster_path.is_some() { 
                    f.poster_path.clone().unwrap()
                } else if f.backdrop_path.is_some() { 
                    f.backdrop_path.clone().unwrap()
                } else { 
                    String::new()
                }
            },
            rated: MediaRated::R.to_string(),
            rating: MovieRating::new(
                Some(f.imdb_id.clone().unwrap_or_default()), 
                Some(f.vote_average.unwrap_or_default() as i32), 
                Some(f.popularity.unwrap_or_default() as f32), 
                Some(f.vote_count.unwrap_or_default().into()), 
                Some(f.vote_average.unwrap_or_default() as f32)
            ),
            release_date: release.clone(),
            runtime:  f.runtime.unwrap_or_default() as i64,
            status: f.status.clone().unwrap_or_default(),
            //  Async request
            video_file: String::new(),
        }
    }
}

/// Get the Movie in this list from the previous list
/// Once we get the list of ids, we want to populate our dataset with movie details 
pub async fn fetch_movies_by_list(
    discover_api: String, 
    endpoint_popular: String, 
    language: Option<String>,
    included_with: Option<String>,
    number_of_batch: Option<i32> 
) -> QueryResult<Vec<i64>> {
    log::info!("👷 Fetching Movie Ids from List");
    let tmdb_url = format!("{api}/{discover_api}/{endpoint_popular}?api_key={key}&language={language}&{included_with}",
                            api = &TMDB_URL,
                            key = TMDB_API_KEY.as_str(),
                            language = language.clone().unwrap_or("en-US".to_string()),
                            included_with = included_with.unwrap_or("".to_string()),
                        );
    let mut batch_items: Vec<i64> = Vec::with_capacity(number_of_batch.unwrap_or(2) as usize);
    let _ = reqwest::get(&tmdb_url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .map(|t: MoviesResponse<TmdbMovie>| t)
        .expect("Unable to process user request")
        .results
        .unwrap_or_default()
        .iter()
        .take(number_of_batch.unwrap_or(2) as usize)
        .for_each(|i| batch_items.push(i.id as i64));
    log::info!("👏 Successs");
    log::info!("Ids to Process: {:#?}", batch_items);
    
    // let concurrent_connection: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(batch_items));
    
    Ok(batch_items)
}

//  Get the the list of all the movies in the right Movie format
pub async fn fetch_movie_details(
    movie_id: Vec<i64>, 
    media_type: String, 
    language: Option<String>
) -> QueryResult<Vec<Movie>> { 
    log::info!("👷 Fetching Movie Details for Each Id in {:#?}", movie_id.clone());
    let mut movie_list = Vec::new();
    let mut keyword: Vec<String> = vec![];
    let mut movie_casts: Vec<String> = vec![];
    let mut directors: Vec<String> = vec![];
    let mut writers: Vec<String> = vec![];
    let mut trailer_id: String = String::new();


    let mut stream = futures::stream::iter(movie_id);
    while let Some(mov_id) = stream.next().await { 
        //https://api.themoviedb.org/3/movie/453395?api_key=82f649eeb0cc9fb9e6ad4785c48623ac&language=en-US
        let tmurl = format!("{url}/movie/{mov_id}?api_key={api}&language={movielanguage}", 
                            url = &TMDB_URL, 
                            api = TMDB_API_KEY.as_str(),
                            movielanguage = language.clone().unwrap_or("en-US".to_string())
        );
        let arc_conn = Arc::new(Mutex::new(mov_id));
        
        log::info!("Executing all future requests");
        //  Wait for multiple different futures to complete while executng themm all concurrently
        let (keyword_,
            casts_, 
            writers_,
            directors_, 
            trailer_id_) = tokio::join!(
            get_keywords(arc_conn.clone()), 
            get_cast(arc_conn.clone()), 
            get_writers(arc_conn.clone()), 
            get_director(arc_conn.clone()), 
            get_trailer(arc_conn.clone())
        );
        keyword = keyword_.expect("Unable to get the keyword");
        log::info!("📦 {:#?}", keyword.clone());
        movie_casts = casts_.expect("Unable to get movie casts");
        log::info!("🤖 {:#?}", movie_casts.clone());

        writers = writers_.expect("Unable to get movie writers");
        log::info!("🚢 {:#?}", writers.clone());

        directors = directors_.expect("Unable to get directors");
        log::info!("🛬 {:#?}", directors.clone());

        trailer_id = trailer_id_.expect("Unable to get trailer id");
        log::info!("📫 {:#?}", trailer_id.clone());
    
        
        let response: String = tokio::spawn( async move {reqwest::get(&tmurl)
            .await
            .expect("Unable to get valid request body")
            .text()
            .await
            .expect("Unable to deserialise the movie")}).await.expect("");
        log::info!("{:#?}", response.clone());
        // temp_serde.push(response);
        // let json_type: MovieDetails = serde_json::from_value(response).unwrap_or_default();
        let json_str: MovieDetails = serde_json::from_str(response.as_str())
            .unwrap_or_default();
    
        log::info!("JSON {:#?}", json_str.clone());
    
        movie_list.push(Movie::from(&json_str));
    }
    //  Converting NewMovie to Movie
    
    log::info!("🏗️🏗️ Checking for any null values in our first check");
    //  2. Aggregate the data into one
    let async_processed = movie_list
    .into_iter()
    .map(|movie: Movie|  { 
        Movie {
            movie_id: movie.movie_id,
            title: movie.title,
            year: movie.year,
            awards: movie.awards,
            business: movie.business,
            countries: movie.countries,
            genres: movie.genres,
            homepage: movie.homepage,
            keywords: keyword.clone(),
            languages: movie.languages,
            media_type: movie.media_type,
            movie_casts: movie_casts.clone(),
            movie_company: movie.movie_company,
            movie_director: directors.clone(),
            movie_writer: writers.clone(),
            overview: movie.overview,
            poster: movie.poster,
            rated: movie.rated,
            rating: movie.rating,
            release_date: movie.release_date,
            runtime: movie.runtime,
            status: movie.status,
            video_file: trailer_id.clone()
        }
    }).collect::<Vec<Movie>>();
    log::info!("📫 Running the final test for our batch call");
    log::info!("👏 Successs");

    Ok(async_processed)
} 

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct KeywordResponse { 
    id: i32, 
    keywords: Option<Vec<Keywords>>
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Keywords { 
    id: i32, 
    name: Option<String>
}

/// Asynchronously gets the keywords under this movie
pub async fn get_keywords(movie_id: Arc<Mutex<i64>>) -> QueryResult<Vec<String>> { 
    let movie_id = *movie_id.lock();
    
    log::info!("👷 Getting Keywords");
    let url = format!("{url}/movie/{movie_id}/keywords?api_key={api}", 
            url = &TMDB_URL, 
            api = TMDB_API_KEY.as_str(),
    );
    //  Take the words 
    let response: KeywordResponse = reqwest::get(&url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .unwrap();
    let keywords = response.keywords.unwrap_or_default();
    let keyword_list = if !keywords.is_empty() { 
        keywords
        .into_iter()
        .map(|f| f.name.unwrap_or_default().clone())
        .collect()
    } else { 
        vec![String::new()]
    };
    Ok(keyword_list)
}

#[derive(Deserialize, Debug, Clone)]
pub struct MovieCast { 
    adult: bool, 
    gender: Option<i32>,
    id: i32, 
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f64,
    profile_path: Option<String>,
    cast_id: i32, 
    character: String,
    credit_id: String,
    order: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct MovieCrew { 
    adult: bool, 
    gender: Option<i32>,
    id: i32, 
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f64,
    profile_path: Option<String>,
    credit_id: String,
    department: String,
    job: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct MovieCreditResponse { 
    id: i32,
    cast: Vec<MovieCast>,
    crew: Vec<MovieCrew>
}

/// Find movie writer asynchronosuly
pub async fn get_writers(movie_id: Arc<Mutex<i64>>) -> QueryResult<Vec<String>> { 
    let movie_id = *movie_id.lock();

    log::info!("👷 Getting Movie Peoples");
    let url = format!(
        "{url}/movie/{movie_id}/credits?api_key={api}&language=en-US",
        url = &TMDB_URL,
        api = TMDB_API_KEY.as_str(),
    );

    // Issue: The items for movie cast is replicated in other sets
    let response: MovieCreditResponse = tokio::spawn( async move{reqwest::get(&url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .map(|t: MovieCreditResponse| t)
        .expect("")}).await.unwrap();
    let get_writers: Vec<String> = response
        .crew
        .into_iter()
        .filter(|f| f.known_for_department == "Writing")
        .map(|n| n.original_name)
        .collect();
    let test_writers = if !get_writers.is_empty() { 
        get_writers
    } else { 
        vec![String::new()]
    };
    Ok(test_writers)
}

#[derive(Deserialize, Debug, Clone)]
pub struct VideosResult { 
    iso_639_1: String,
    iso_3166_1: String,
    name: String,
    key: String,
    site: String,
    size: i64,
    #[serde(rename = "type")]
    video_type: String,
    official: bool, 
    published_at: String,
    id: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct VideoResponse { 
    id: i32,
    results: Option<Vec<VideosResult>>
}
pub async fn get_trailer(movie_id: Arc<Mutex<i64>>) -> QueryResult<String> { 
    let movie_id = *movie_id.lock();

    log::info!("👷 Getting Movie Trailers ");

    let url = format!(
        "{url}/movie/{movie_id}/videos?api_key={api}&language=en-US&append_to_response=videos",
        url = &TMDB_URL,
        api = TMDB_API_KEY.as_str(),
    );

    let video_response: Vec<VideosResult> = reqwest::get(&url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .map(|t: VideoResponse| t)
        .expect("")
        .results
        .unwrap_or_default();
    let video_id = video_response 
        .into_iter()
        .filter(|r| r.video_type == "Trailer" )
        .map(|r| r.id.clone()).collect();
    Ok(video_id)
    
}

/// Find the movie actors asynchronously
pub async fn get_cast(movie_id: Arc<Mutex<i64>>) -> QueryResult<Vec<String>> { 
    let movie_id = *movie_id.lock();

    log::info!("👷 Getting Movie actors");
    let url = format!(
        "{url}/movie/{movie_id}/credits?api_key={api}&language=en-US",
        url = &TMDB_URL,
        api = TMDB_API_KEY.as_str(),
    );
    // Issue: The items for movie cast is replicated in other sets
    let response: MovieCreditResponse = reqwest::get(&url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .map(|t: MovieCreditResponse| t)
        .expect("");
    let casts: Vec<String> = response
        .cast
        .clone()
        .into_iter()
        .filter(|f| f.known_for_department == "Acting")
        .map(|n| n.character)
        .collect(); 
    let test_cast = if !casts.is_empty() { 
        casts.clone()
    } else { 
        vec![String::new()]
    };
    Ok(test_cast)
}
/// Find the movie director asynchronously
pub async fn get_director(movie_id: Arc<Mutex<i64>>) -> QueryResult<Vec<String>>{ 
    let movie_id = *movie_id.lock();

    log::info!("👷 Getting Movie actors");
    let url = format!(
        "{url}/movie/{movie_id}/credits?api_key={api}&language=en-US",
        url = &TMDB_URL,
        api = TMDB_API_KEY.as_str(),
    );
    // Issue: The items for movie cast is replicated in other sets
    let response: MovieCreditResponse = reqwest::get(&url)
        .await
        .expect("Unable to get valid request body")
        .json()
        .await
        .map(|t: MovieCreditResponse| t)
        .expect("");
    let get_directors: Vec<String> = response
        .crew.clone()
        .into_iter()
        .filter(|f| f.known_for_department == "Directing")
        .map(|n| n.original_name)
        .collect();

    let test_directors = if !get_directors.is_empty() { 
        get_directors.clone()
    } else { 
        vec![String::new()]
    };
    Ok(test_directors)
}
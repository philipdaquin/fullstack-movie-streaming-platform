use async_trait::async_trait;
use common_utils::QueryResult;
use scylla::{IntoTypedRows, prepared_statement::PreparedStatement};
use crate::db::CachedSession;
use super::model::{NewMovie, Movie}; 
use scylla::batch::Batch;
use futures::StreamExt;

#[async_trait]
pub trait MovieResolver { 
    async fn get_movie_id(id: i64, session: &'static CachedSession) -> QueryResult<Movie>;
    async fn create_movie(new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie>;
    async fn update_movie(id: i64, new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie>;
    async fn delete_movie(id: i64, title: String, session: &'static CachedSession) -> QueryResult<bool>;
    async fn bulk_insert(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool>; 
    async fn stream_insert(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool>;
}

pub struct MovieDatabase;

static CREATE_MOVIE: &str = "
    INSERT INTO movie_keyspace.movies_object (
        movie_id, title, year, awards, business, countries, genres, homepage, 
        keywords, languages, media_type, movie_casts, movie_company, movie_director, 
        movie_writer, overview, poster, rated, rating, release_date, runtime, status, 
        video_file 
    ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
";
static DELETE_MOVIE: &str = "DELETE FROM movie_keyspace.movies_object WHERE movie_id = ? AND title = ?;";
static UPDATE_MOVIE: &str = "UPDATE movie_keyspace.movies_object SET 
        title = new_title, 
        year = new_year, 
        awards = new_award, 
        business = new_business, 
        countries = new_couontries, 
        genres = new_genres, 
        homepage = new_homepage, 
        keywords = new_keywords, 
        languages = new_languages, 
        media_type = new_media_type, 
        movie_casts = movie_casts, 
        movie_company = new_company, 
        movie_director = new_movie_director, 
        movie_writer = new_movie_writer, 
        overview = new_overview, 
        poster = new_poster, 
        rated = new_rated, 
        rating = new_rating, 
        release_date = new_release_date, 
        runtime = new_runtime, 
        status = new_status, 
        video_file = new_video_file
    WHERE movie_id = ?
        AND title <> new_title,
        AND year <> new_year, 
        AND awards <> new_award, 
        AND business <> new_business, 
        AND countries <> new_couontries, 
        AND genres <> new_genres, 
        AND homepage <> new_homepage, 
        AND keywords <> new_keywords, 
        AND languages <> new_languages, 
        AND media_type <> new_media_type, 
        AND movie_casts <> movie_casts, 
        AND movie_company <> new_company, 
        AND movie_director <> new_movie_director, 
        AND movie_writer <> new_movie_writer, 
        AND overview <> new_overview, 
        AND poster <> new_poster, 
        AND rated <> new_rated, 
        AND rating <> new_rating, 
        AND release_date <> new_release_date, 
        AND runtime <> new_runtime, 
        AND status <> new_status, 
        AND video_file <> new_video_file;
";

#[async_trait]
impl MovieResolver for MovieDatabase { 
    /// Gets a movie using the Movie id 
    #[tracing::instrument(skip(session), fields(repository = "asset_ingestion.movies_object"))]
    async fn get_movie_id(id: i64, session: &'static CachedSession) -> QueryResult<Movie> {
        let response = session.query_prepared("SELECT * FROM movie_keyspace.movies_object WHERE movie_id = ? ALLOW FILTERING;", (id,))
            .await
            .expect("")
            .rows
            .unwrap_or_default();
        log::info!("RESPONSE 😂😂 {:#?}", response);
        let res = response 
            .into_typed::<Movie>()
            .next()
            .expect("")
            .expect("");
        Ok(res)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"), err)]
    async fn create_movie(new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie> {
        log::info!("ENTERING THE DATABASE {:#?}", new_movie);
        let response = session
            .query_prepared(CREATE_MOVIE, new_movie.clone())
            .await
            .expect("Unable to create new Movie");
        log::info!("Database Response {:#?}", response);
       MovieDatabase::get_movie_id(new_movie.movie_id, session).await        
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"), err)]
    async fn update_movie(id: i64, new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie> {
        let response = session
            .query_prepared(UPDATE_MOVIE, (
                new_movie,
                id))
            .await
            .expect("Unable to update the movie")
            .rows
            .unwrap_or_default()
            .into_typed::<Movie>()
            .next()
            .unwrap()
            .unwrap();
        Ok(response)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"), err)]
    async fn delete_movie(id: i64, title: String, session: &'static CachedSession) -> QueryResult<bool> {
        let res = session
            .query_prepared(DELETE_MOVIE, (id, title ))
            .await
            .unwrap_or_default();
        log::info!("Deleted Result: {:#?}", res);
        Ok(true)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"), err)]
    async fn bulk_insert(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool> { 
        let mut batch = Batch::default();
        log::info!("👀 Preparing to make batch call for {:#?}", movie);
        let len = movie.len();
        // Build all the statements first
        for _ in 0..len { 
            let prepared: PreparedStatement = session
                .0
                .session
                .prepare(CREATE_MOVIE)
                .await
                .expect("Unable to batch prepared statments together");
            batch.append_statement(prepared);
        }
        batch.set_tracing(true);
        //  Insert each UDTs as our values 
        session
            .0
            .session
            .batch(&batch, (movie))
            .await
            .expect("Error Executing the batch statement");
        log::info!("Reached the end of batch Query");
        log::info!("🤖 Batch Tracing Info: {:#?}", batch.get_tracing());
        Ok(true)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"), err)]
    async fn stream_insert(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool> { 
        let mut stream = futures::stream::iter(movie);
        while let Some(movie) = stream.next().await { 
            log::info!("🛬 Streaming {:#?} into the Database ", movie.clone());
            let res = session.query_prepared(CREATE_MOVIE, (movie))
                .await
                .expect("Unable to stream movies");
        }
        Ok(true)
    }

}
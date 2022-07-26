-- Represents the movie company information 
CREATE TABLE IF NOT EXISTS movie_keyspace.movie_company (
    company_id BIGINT,
    name TEXT,
    description TEXT,
    logo_path TEXT,
    headquarter TEXT,
    homepage TEXT,
    origin_country TEXT,
    parent_company TEXT,
    movie_id BIGINT,
    PRIMARY KEY (company_id, name)
) WITH CLUSTERING ORDER BY (name ASC);

-- Person information 
-- Used for querying the movie writer, cast, director to give  the user more details 
-- when they request for more information about them 
CREATE TABLE IF NOT EXISTS movie_keyspace.person_object (
    person_id INT,
    known_for SET<TEXT>,
    name TEXT,
    birthday DATE,
    death_date DATE,
    place_of_birth TEXT,
    gender TEXT,
    biography TEXT,
    profile_path LIST<TEXT>,
    homepage TEXT,
    awards SET<TEXT>,
    PRIMARY KEY (person_id, name)
) WITH CLUSTERING ORDER BY (name ASC);

-- Represents the Movie's Ratings
-- Chose to be frozen type since rating will always be kept up to date
CREATE TYPE IF NOT EXISTS movie_keyspace.movie_ratings (
    imdb_id TEXT,
    metascore INT,
    popularity FLOAT,
    vote_count BIGINT,
    vote_average FLOAT
);
-- Information of Movie budget and revenues
CREATE TYPE IF NOT EXISTS movie_keyspace.business_data (
    budget BIGINT,
    revenue BIGINT,
);

-- Represents the Movie Data
-- User lists will reference the movie_id and name 
-- Elasticsearch can be used to filter user search queries based on genres
CREATE TABLE IF NOT EXISTS movie_keyspace.movies_object (
    movie_id BIGINT,              
    title TEXT,             
    year INT,
    rated TEXT,            -- R, PG
    release_date DATE,      
    homepage TEXT,
    overview TEXT,
    poster TEXT,    
    media_type TEXT,        -- Video, Clip, Trailer, Movie, Series
    video_file TEXT,        -- S3 Link
    runtime BIGINT,
    status TEXT,
    rating FROZEN<movie_ratings>, 
    business FROZEN<business_data>,
    languages SET<TEXT>,           
    countries SET<TEXT>,           
    movie_company SET<TEXT>,       
    genres SET<TEXT>,              
    keywords SET<TEXT>,            
    movie_casts SET<TEXT>,         
    movie_writer SET<TEXT>,
    movie_director SET<TEXT>,
    awards SET<TEXT>,
    PRIMARY KEY ((movie_id, title), year)
) WITH CLUSTERING ORDER BY (year DESC);



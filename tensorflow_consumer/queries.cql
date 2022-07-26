
--- RECORD THE 
CREATE TABLE IF NOT EXISTS recommended_movies.user_recommendations (
    user_id INT,
    movie_id BIGINT,
    title TEXT,
    created_at DATE,    -- custom write to each item 
    PRIMARY KEY (user_id, created_at) 
) WITH CLUSTERING ORDER BY (created_at DESC);



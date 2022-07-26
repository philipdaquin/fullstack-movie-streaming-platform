-- Your SQL goes here

ALTER TABLE products 
ADD    category VARCHAR(128),
ADD    created_by INT,
ADD    tags VARCHAR(64),
ADD    created_at TIMESTAMP DEFAULT NOW(),
ADD    updated_at TIMESTAMP,
ADD    description VARCHAR,
ADD    image_url VARCHAR(250);
    
    
   
    
    
    
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use serde_json::json;

// These are temporary values, they are subject to change as the project progresses 
lazy_static! { 
    pub(crate) static ref MOVIE_MAPPING: serde_json::Value = json!({
        "analysis": {
            "char_filter": {
               "replace": {
                "type": "mapping",
                "mappings": [
                  "&=> and "
                ]
              }
            },
            "filter": {
              "word_delimiter" : {
                "type" : "word_delimiter",
                "split_on_numerics" : false,
                "split_on_case_change" : true,
                "generate_word_parts" : true,
                "generate_number_parts" : true,
                "catenate_all" : true,
                "preserve_original":true,
                "catenate_numbers":true
              }
            },
            "analyzer": {
              "default": {
                "type": "custom",
                "char_filter": [
                  "html_strip",
                  "replace"
                ],
                "tokenizer": "whitespace",
                "filter": [
                    "lowercase",
                    "word_delimiter"
                ]
              }
            }
          }
    });
    pub(crate) static ref MOVIE_SETTING: serde_json::Value = json!({
        "mappings" : {
            "properties" : {
                "movie_id" : {
                    "type" : "text"
                },
                "title" : {
                    "type" : "text",
                    "fields": {
                        "raw": {
                            "type": "keyword"
                        },
                        "english": {
                            "type": "text",
                            "analyzer": "english"
                        }
                    }
                },
                "keywords": {
                    "type": "text",
                    "fields": {
                        "english": {
                            "type": "text",
                            "analyzer": "english"
                        }
                    }
                },
                "overview": {
                    "type": "text",
                    "fields": {
                        "english": {
                            "type": "text",
                            "analyzer": "english",
                            "ignore_above": 256
                        }
                    }
                },
                "release_date": {
                    "type": "date",
                    "format": "yyyy-MM-dd||epoch_millis"
                },
                "languages": {
                    "properties": {
                        "code": {
                            "type": "keyword"
                        },
                        "name": {
                            "type": "text",
                            "fields": {
                                "raw": {
                                    "type": "keyword"
                                }
                            }
                        }
                    }
                },
                "countries": {
                    "properties": {
                        "code": {
                            "type": "keyword"
                        },
                        "name": {
                            "type": "text",
                            "fields": {
                                "raw": {
                                    "type": "keyword"
                                }
                            }
                        }
                    }
                },
                "genres": {
                    "properties": {
                        "name": {
                            "type": "text",
                            "fields": {
                                "raw": {
                                    "type": "keyword"
                                }
                            }
                        }
                    }
                },
                "movie_director": {
                    "properties": {
                        "name": {
                            "type": "text",
                            "fields": {
                                "raw": {
                                    "type": "keyword"
                                }
                            }
                        }
                    }
                },
                "movie_writer": {
                    "properties": {
                        "name": {
                            "type": "text",
                            "fields": {
                                "raw": {
                                    "type": "keyword"
                                }
                            }
                        }
                    }
                },
                "movie_director": {
                    "type": "text"
                },
                "created": {
                    "type": "date"
                },
                "updated": {
                    "type": "date"
                },
                "indexed": {
                    "type": "date"
                }
            }
        }
    });
}

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Movie { 
    pub movie_id: i64,
    pub title: String,
    pub year: i32,
    pub awards: Vec<String>,
    pub business: BusinessData,
    pub countries: Vec<String>,
    pub genres: Vec<String>,
    pub homepage: String,
    pub keywords: Vec<String>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub movie_casts: Vec<String>,
    pub movie_company: Vec<String>,
    pub movie_director: Vec<String>,
    pub movie_writer: Vec<String>,
    pub overview: String,
    pub poster: String,
    pub rated: String,
    pub rating: MovieRating,
    pub release_date: NaiveDate,
    pub runtime: i64,
    pub status: String,
    pub video_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct BusinessData { 
    pub budget: i64, 
    pub revenue: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct MovieRating { 
    pub imdb_id: String,
    pub metascore: i32, 
    pub popularity: f32,
    pub vote_count: i64, 
    pub vote_average: f32
}

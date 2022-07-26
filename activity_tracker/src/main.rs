use activity_tracker::server::new_server;
use failure::Error;


#[actix_web::main]
async fn main() -> Result<(), Error> { 
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(4008);
        
    new_server(port)
        .await
        .map_err(Into::into)
}
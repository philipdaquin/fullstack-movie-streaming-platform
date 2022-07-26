
use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder, HttpResponse, HttpRequest, guard};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema, Context, extensions::ApolloTracing,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::db::{DbPool, DbPooledConnection};
use super::root_schema::{Mutation, Query, AppSchema, AppSchemaBuilder};
use diesel::{result::Error as DbError, QueryDsl};
use diesel_migrations::{MigrationError, embed_migrations};

pub fn configure_service(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(graphql)
    .service(graphql_playground)
    .service(
        web::resource("/graphiql")
            .route(web::get()
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws)
        )
    );
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
/// GraphiQL playground UI
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

pub async fn index_ws(
    schema: web::Data<AppSchema>, 
    req: HttpRequest, 
    payload: web::Payload) -> Result<HttpResponse, actix_web::Error> { 
    GraphQLSubscription::new(Schema::clone(&*schema))
        .start(&req, payload)
}

embed_migrations!();

pub fn create_schema(pool: DbPool) -> AppSchema { 
    Schema::build(
        Query::default(), 
        EmptyMutation, 
        EmptySubscription
    )
    // Add a global data that can be accessed in the Schema
    .data(pool)
    .extension(ApolloTracing)
    .finish()
}
pub fn run_migrations(pool: &DbPool) { 
    let conn = pool.get().expect("Database Connection Pool - Migrations error!");
    embedded_migrations::run(&conn).expect("Failed to run database migrations");
}
pub fn get_conn_from_ctx(ctx: &Context<'_>) -> DbPooledConnection { 
    ctx.data::<DbPool>()
        .expect("Failed to get Db Pool")
        .get()
        .expect("Failed to Connect to Database")
}
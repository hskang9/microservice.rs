#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetrics;
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

pub mod db;
pub mod graphql_schema;
pub mod schema;
pub mod utils;

use crate::db::establish_connection;
use crate::graphql_schema::{create_schema, Context, Schema};

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:5001/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let pool = establish_connection();
    let schema_context = Context { db: pool.clone() };
    let schema = std::sync::Arc::new(create_schema());
    let prometheus = PrometheusMetrics::new("api", "/metrics");
    println!("Server running at http://0.0.0.0:5001");
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .data(schema.clone())
            .data(schema_context.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("0.0.0.0:5001")?
    .run()
}

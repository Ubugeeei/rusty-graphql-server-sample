use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// tentative db
static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));
static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

/*
 * Types
 */
#[allow(dead_code)]
#[derive(SimpleObject, Clone)]
struct Todo {
    id: usize,
    title: String,
    description: String,
    is_done: bool,
    due_date: Option<String>,
}
impl Todo {
    fn new(title: String, description: String, due_date: Option<String>) -> Todo {
        let mut id = SEQUENCE_ID.lock().unwrap();
        *id += 1;
        Todo {
            id: *id,
            title,
            description,
            is_done: false,
            due_date,
        }
    }
}

/*
 * Queries
 */
struct Query;
#[Object]
impl Query {
    async fn static_value(&self) -> usize {
        42
    }

    async fn get_todos(&self) -> Vec<Todo> {
        TODOS.lock().unwrap().clone()
    }
}

/*
 * Mutations
 */
struct Mutation;
#[Object]
impl Mutation {
    async fn create_todo(
        &self,
        title: String,
        description: String,
        due_date: Option<String>,
    ) -> bool {
        let todo = Todo::new(title, description, due_date);
        TODOS.lock().unwrap().push(todo);
        true
    }
}

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("listen ...");
    println!("http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

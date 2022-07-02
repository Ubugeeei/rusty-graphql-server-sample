use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Query;
#[Object]
impl Query {
    async fn static_value(&self) -> usize {
        42
    }
}

static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

#[allow(dead_code)]
struct Todo {
    title: String,
    description: String,
    is_done: bool,
    due_date: Option<String>,
}
impl Todo {
    fn new(title: String, description: String, due_date: Option<String>) -> Todo {
        Todo {
            title,
            description,
            is_done: false,
            due_date,
        }
    }
}

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

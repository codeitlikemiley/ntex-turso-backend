use dotenvy::dotenv;
use libsql::{de, params, Builder, Connection};
use ntex::{
    util::Either,
    web::{
        get, post, put,
        types::{Json, Path, State},
        App, Error, HttpResponse, HttpServer, Responder,
    },
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::Duration};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: Box<str>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateUser {
    name: Box<str>,
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdateUser {
    name: Box<str>,
}

type UserResponse = Either<HttpResponse, Result<Vec<User>, Error>>;

#[put("/users/{id}")]
async fn update_user(
    data: State<Arc<Db>>,
    path: Path<u32>,
    Json(payload): Json<UpdateUser>,
) -> impl Responder {
    let conn = &data.conn;
    let user_id = path.into_inner();

    let result = conn
        .query(
            "UPDATE users SET name = ?1 WHERE id = ?2",
            params![payload.name.clone(), user_id],
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User updated successfully"),
        Err(e) => {
            println!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[get("/users")]
async fn index_user(data: State<Arc<Db>>) -> impl Responder {
    let results = data.conn.query("SELECT * FROM users", ()).await;

    match results {
        Ok(mut rows) => {
            let mut users: Vec<User> = Vec::new();

            while let Some(row) = rows.next().await.unwrap() {
                let user: User = User {
                    name: Box::from(row.get_str(1).unwrap()),
                };
                users.push(user);
            }

            HttpResponse::Ok().json(&users)
        }
        Err(e) => {
            println!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[get("/users/{id}")]
async fn get_user(data: State<Arc<Db>>, path: Path<u32>) -> impl Responder {
    let mut stmt = data
        .conn
        .prepare("SELECT * FROM users WHERE id = ?1")
        .await
        .unwrap();

    let row = stmt
        .query([path.into_inner()])
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();

    let user = de::from_row::<User>(&row).unwrap();

    HttpResponse::Ok().json(&user)
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/users")]
async fn create_user(Json(payload): Json<CreateUser>) -> impl Responder {
    let user = User { name: payload.name };

    let conn = connection().await;

    let _ = conn
        .query(
            "INSERT into users (name) values (?)",
            params![user.name.clone()],
        )
        .await;

    HttpResponse::Created()
}

struct Db {
    conn: libsql::Connection,
}

async fn connection() -> Connection {
    dotenv().expect(".env file not found");

    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote_replica("local.db", url, token)
        .read_your_writes(true)
        .sync_interval(Duration::from_secs(5))
        .build()
        .await
        .unwrap();

    // this can update local database
    // let db = Builder::new_local("local.db").build().await.unwrap();

    db.connect().unwrap()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let conn = connection().await;

    let db_state = Arc::new(Db { conn });

    HttpServer::new(move || {
        App::new()
            .state(db_state.clone())
            .service(home)
            .service(update_user)
            .service(index_user)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

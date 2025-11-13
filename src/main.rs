use std::thread;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio;
use sea_orm::{Database, DatabaseConnection, ActiveValue, ActiveModelTrait};

// use migration::{Migrator, MigratorTrait};


fn print_type_of<T>(obj: &T){
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug, Clone)]
struct MyState{
    db: DatabaseConnection
}

use my_db::user::ActiveModel as UserModel;
#[get("/")]
async fn add_user(config: web::Data<MyState>) -> impl Responder {

    // name.push_str("asdf");
    // print_type_of(&*name);
    let user = UserModel{
        username: ActiveValue::set(String::from("user1")),
        ..<UserModel as Default>::default()
    };
    user.save(&config.db).await.unwrap();
    // print_type_of(&user.username.unwrap());
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn hello(config: web::Data<MyState>) -> impl Responder {

    // name.push_str("asdf");
    // print_type_of(&*name);
    let user = UserModel{
        username: ActiveValue::set(String::from("user1")),
        ..<UserModel as Default>::default()
    };
    user.save(&config.db).await.unwrap();
    // print_type_of(&user.username.unwrap());
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     }).workers(1)
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mystate = MyState{
        db:database_connection().await
    };
    HttpServer::new(move || {
        let mut name = String::from("Roni");
        App::new()
            .app_data(web::Data::new(mystate.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod my_db;
async fn database_connection()->DatabaseConnection{
    let db = Database::connect("sqlite://db.sqlite3?mode=rwc").await.unwrap();
    println!("asfasdf");
    // db.get_schema_registry("learn_actix::db::*").sync(&db).await.unwrap();
    // Migrator::up(&db, None).await.unwrap();
    db.get_schema_registry("learn_actix::my_db::*").sync(&db).await.unwrap();
    return db;
}

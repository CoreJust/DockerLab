use dotenv::dotenv;
use sea_orm::*;
use std::env;
use tide::{Body, Request, Response};

mod models;
mod service;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL environment variable provided");
    let db = Database::connect(&db_url).await?;
    println!("Connected to PostgresQL DB at {db_url}");

    let mut app = tide::with_state(db);
    app.at("/").get(|_| async move {
        println!("Getting index");
        Ok("Enter add_user?name=<name>&email=<email> to add a user, user?id=<id> to get user, users to get all users, and remove_user?id=<id> to remove a user.")
    });
    app.at("/add_user")
        .get(|req: Request<DatabaseConnection>| async move {
            let user: service::User = req.query()?;
            println!("Adding user {} ({})", user.name, user.email);
            match service::put_user(user, &req.state()).await {
                Err(e) => Err(tide::Error::from_str(500, e.to_string())),
                Ok(_) => Ok(Response::builder(200).build()),
            }
        });
    app.at("/user")
        .get(|req: Request<DatabaseConnection>| async move {
            let service::UserID { id } = req.query()?;
            println!("Getting user with id {id}");
            match service::get_user(id, &req.state()).await {
                Err(e) => Err(tide::Error::from_str(500, e.to_string())),
                Ok(user) => match user {
                    None => Ok(Response::builder(404)
                        .body(format!("User with id {} not found", id))
                        .build()),
                    Some(user) => Ok(Response::builder(200).body(Body::from_json(&user)?).build()),
                },
            }
        });
    app.at("/users")
        .get(|req: Request<DatabaseConnection>| async move {
            println!("Getting all users");
            match service::get_all_users(&req.state()).await {
                Err(e) => Err(tide::Error::from_str(500, e.to_string())),
                Ok(users) => Ok(Response::builder(200)
                    .body(Body::from_string(
                        users
                            .into_iter()
                            .map(|x| format!("User {}; id {}; email {}\n", x.name, x.id, x.email))
                            .collect::<Vec<String>>()
                            .join(""),
                    ))
                    .build()),
            }
        });
    app.at("/remove_user")
        .get(|req: Request<DatabaseConnection>| async move {
            let service::UserID { id } = req.query()?;
            println!("Removing user with id {id}");
            match service::delete_user(id, &req.state()).await {
                Err(e) => Err(tide::Error::from_str(500, e.to_string())),
                Ok(_) => Ok(Response::builder(200).build()),
            }
        });

    let app_port = env::var("PORT").expect("No PORT environment variable provided");
    println!("Application is listening on 127.0.0.1:{app_port}");
    app.listen(format!("0.0.0.0:{app_port}")).await?;
    Ok(())
}

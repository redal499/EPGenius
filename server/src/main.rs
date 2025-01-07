// ...existing code...

mod db;
mod models;
mod handlers;
mod routes;

use warp::Filter;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    db::initialize_db();

    // Заглушка для ежедневного обновления EPG (примерно раз в сутки)
    tokio::spawn(async {
        loop {
            handlers::daily_epg_update();
            sleep(Duration::from_secs(86400)).await;
        }
    });

    println!("Running warp on 127.0.0.1:3030");
    warp::serve(routes::routes()).run(([127, 0, 0, 1], 3030)).await;
}
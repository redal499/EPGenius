// ...existing code...

use crate::db::get_db_conn;
use crate::models::*;
use rusqlite::{params, Connection};
use warp::http::StatusCode;

// Регистрация
pub async fn register_user_handler(body: UserRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_db_conn();
    let mut stmt = conn.prepare("INSERT INTO users (name,email,password,date_of_birth) VALUES (?,?,?,?)").unwrap();
    match stmt.execute(params![
        body.name.unwrap_or_default(),
        body.email,
        body.password,
        body.date_of_birth.unwrap_or_default()
    ]) {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&"Registered"),
            StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: {:?}", e)),
            StatusCode::BAD_REQUEST,
        )),
    }
}

// Логин
pub async fn login_handler(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_db_conn();
    let mut stmt = conn
        .prepare("SELECT id,email,password FROM users WHERE email=?")
        .unwrap();
    let mut rows = stmt.query(params![body.email]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        let db_pass: String = row.get(2).unwrap();
        if db_pass == body.password {
            let user_id: i64 = row.get(0).unwrap();
            let email: String = row.get(1).unwrap();
            let resp = UserResponse { id: user_id, email };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK,
            ));
        }
    }
    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
            message: "Invalid credentials".into(),
        }),
        StatusCode::UNAUTHORIZED,
    ))
}

// Интересы - получить
pub async fn get_interests_handler(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_db_conn();
    let mut stmt = conn.prepare("SELECT id,interest FROM interests WHERE user_id=?").unwrap();
    let mut rows = stmt.query(params![user_id]).unwrap();
    let mut interests = vec![];
    while let Some(row) = rows.next().unwrap() {
        interests.push(InterestResponse{
            id: row.get(0).unwrap(),
            interest: row.get(1).unwrap(),
        });
    }
    Ok(warp::reply::json(&interests))
}

// Интересы - добавить
pub async fn add_interest_handler(user_id: i64, body: InterestRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_db_conn();
    conn.execute("INSERT INTO interests (user_id, interest) VALUES (?, ?)", params![user_id, body.interest]).unwrap();
    Ok(warp::reply::json(&"ok"))
}

// Интересы - удалить
pub async fn delete_interest_handler(user_id: i64, interest_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_db_conn();
    conn.execute("DELETE FROM interests WHERE id=? AND user_id=?", params![interest_id, user_id]).unwrap();
    Ok(warp::reply::json(&"deleted"))
}

// Заглушка для ежедневного скачивания и актуализации EPG
pub fn daily_epg_update() {
    // Здесь логика для cron
    println!("EPG updated daily");
}

// Заглушка парсинга EPG
pub async fn parse_epg_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // Логика чтения epg.xml, сохранение в БД
    Ok(warp::reply::json(&"EPG parsed"))
}

// Заглушка для нейросети
pub async fn neural_request_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // Логика отправки запроса к нейросети
    Ok(warp::reply::json(&"Neural response"))
}
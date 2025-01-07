// ...existing code...

use warp::{Filter, Rejection, Reply};
use crate::handlers::*;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register_user_handler);

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let get_interests = warp::path!("interests" / i64)
        .and(warp::get())
        .and_then(get_interests_handler);

    let add_interest = warp::path!("interests" / i64)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_interest_handler);

    let delete_interest = warp::path!("interests" / i64 / i64)
        .and(warp::delete())
        .and_then(delete_interest_handler);

    let parse_epg = warp::path("parse_epg")
        .and(warp::post())
        .and_then(parse_epg_handler);

    let neural_req = warp::path("neural")
        .and(warp::post())
        .and_then(neural_request_handler);

    // Собираем роуты
    register
        .or(login)
        .or(get_interests)
        .or(add_interest)
        .or(delete_interest)
        .or(parse_epg)
        .or(neural_req)
}
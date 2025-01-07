//models.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub name: Option<String>,
    pub email: String,
    pub password: String,
    pub date_of_birth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct InterestRequest {
    pub interest: String,
}

#[derive(Serialize, Deserialize)]
pub struct InterestResponse {
    pub id: i64,
    pub interest: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}
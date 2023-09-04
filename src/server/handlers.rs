use super::auth::*;
use super::error::Error::*;
use super::mongo::{Db, Expense, User};
use mongodb::bson::{self, doc, Document};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reject::{self, reject, Rejection};
use warp::reply::{self, Reply};

pub async fn list_expenses(user_id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub async fn add_expense(
    user_id: String,
    expense: Expense,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::CREATED)
}

pub async fn add_income(
    user_id: String,
    income: Expense,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::CREATED)
}

pub async fn add_user(user: User, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("add_user: {:?}", user);
    let serialized_user = bson::to_bson(&user).unwrap();
    let document = serialized_user.as_document().unwrap();
    let users = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("users");
    let user_found = users
        .find_one(
            doc! {
                "email": user.email,
            },
            None,
        )
        .await
        .unwrap();
    if user_found != None {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let insert_result = users.insert_one(document.to_owned(), None).await;
    if insert_result.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::CREATED)
}

pub async fn delete_expense(
    expense_id: u64,
    user_id: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn user_login(login_info: LoginInfo, db: Db) -> Result<impl warp::Reply, Rejection> {
    if verify_user(login_info.clone(), db).await {
        let token = gen_token(&login_info.email).map_err(|e| reject::custom(e))?;
        Ok(reply::json(&LoginResponse { token }))
    } else {
        Err(reject::custom(WrongCredentialsError))
    }
}

pub async fn auth_test(user_id: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("hello user {}", user_id))
}

use super::mongo::{Expense, User, Db, LoginInfo};
use super::authenticate::{*};
use std::convert::Infallible;
use mongodb::bson::{Document, self, doc};
use warp::http::StatusCode;

pub async fn list_expenses(user_id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    if is_authenthicated(user_id, db).await {
        Ok(StatusCode::OK)
    }
    else {
        Ok(StatusCode::FORBIDDEN)
    }
}

pub async fn add_expense(user_id: String, expense: Expense, db: Db) -> Result<impl warp::Reply, Infallible> {
    if is_authenthicated(user_id, db).await {
        Ok(StatusCode::CREATED)
    }
    else {
        Ok(StatusCode::FORBIDDEN)
    }
}

pub async fn add_income(user_id: String, income: Expense, db: Db) -> Result<impl warp::Reply, Infallible> {
    if is_authenthicated(user_id, db).await {
        Ok(StatusCode::CREATED)
    }
    else {
        Ok(StatusCode::FORBIDDEN)
    }
}

pub async fn add_user(user: User, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("add_user: {:?}", user);
    let serialized_user = bson::to_bson(&user).unwrap();
    let document = serialized_user.as_document().unwrap();
    let users = db.client.unwrap().database("budget_boi").collection::<Document>("users");
    let user_found = users.find_one(
        doc! {
            "email": user.email,
        },
        None,
    ).await.unwrap();
    if user_found != None {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let insert_result = users.insert_one(document.to_owned(), None).await;
    if insert_result.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
    Ok(StatusCode::CREATED)
}

pub async fn delete_expense(user_id: String, expense_id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    if is_authenthicated(user_id, db).await {
        Ok(StatusCode::NO_CONTENT)
    }
    else {
        Ok(StatusCode::UNAUTHORIZED)
    }
}

pub async fn user_login(login_info: LoginInfo, db: Db) -> Result<impl warp::Reply, Infallible> {
    //TODO - implement login functionality along with function below
    if verify_user(login_info, db).await {
        Ok(StatusCode::OK)
    }
    else {
        Ok(StatusCode::UNAUTHORIZED)
    }
    
}

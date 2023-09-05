use super::auth::*;
use super::error::Error::*;
use super::mongo::{Db, Expense, ExpenseRequest, User};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, Document};
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::reject::{self, reject, Rejection};
use warp::reply::{self, Reply};

use tokio_stream::StreamExt;

pub async fn list_expenses(user_id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let expenses = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("expenses");
    let mut cursor = expenses
        .find(
            doc! {
            "owner": user_id,
            },
            None,
        )
        .await
        .unwrap();
    let mut expenses: Vec<ExpenseRequest> = vec![];
    while let Some(doc) = cursor.next().await {
        let expense: ExpenseRequest = bson::from_bson(mongodb::bson::Bson::Document(doc.unwrap())).unwrap();
        expenses.push(expense);
    }
    Ok(warp::reply::json(&expenses))
}

pub async fn add_expense(
    user_id: String,
    expense: Expense,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let expense_request = ExpenseRequest::new(expense, user_id);
    let serialized_expense = bson::to_bson(&expense_request).unwrap();
    let document = serialized_expense.as_document().unwrap();
    let expenses = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("expenses");
    let insert_result = expenses.insert_one(document.to_owned(), None).await;
    if insert_result.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }
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
    let id = verify_user(login_info.clone(), db).await;
    if id != None {
        let token = gen_token(&id.unwrap().to_string()).map_err(|e| reject::custom(e))?;
        Ok(reply::json(&LoginResponse { token }))
    } else {
        Err(reject::custom(WrongCredentialsError))
    }
}

pub async fn auth_test(user_id: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("hello user {}", user_id))
}

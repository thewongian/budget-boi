use super::mongo::{Expense, User, Db};
use std::convert::Infallible;
use mongodb::bson::{Document, self, doc};
use warp::http::StatusCode;

pub async fn list_expenses(user_id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub async fn add_expense(expense: Expense, db: Db) -> Result<impl warp::Reply, Infallible> {

    Ok(StatusCode::CREATED)
}

pub async fn add_income(income: Expense, db: Db) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::CREATED)
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



pub async fn delete_expense(user_id: u64, expense_id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::NO_CONTENT)
}

use super::mongo::{Expense, User};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_expenses(user_id: u64) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub async fn add_expense(expense: Expense) -> Result<impl warp::Reply, Infallible> {

    Ok(StatusCode::CREATED)
}

pub async fn add_income(income: Expense) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::CREATED)
}

pub async fn add_user(user: User) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::CREATED)
}

pub async fn delete_expense(user_id: u64, expense_id: u64) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::NO_CONTENT)
}

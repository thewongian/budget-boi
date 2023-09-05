use super::auth::*;
use super::error::Error::*;
use super::mongo::{Db, Expense, ExpenseList, User};
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
    let mut expenses: Vec<Expense> = vec![];
    while let Some(doc) = cursor.next().await {
        let mut expense: ExpenseList = bson::from_bson(mongodb::bson::Bson::Document(doc.unwrap())).unwrap();
        expenses.append(&mut expense.list);
    }
    Ok(warp::reply::json(&expenses))
}

pub async fn add_expense(
    user_id: String,
    mut expense: Expense,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    
    let expenses = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("expenses");
    let expenses_found = expenses.find_one(
        doc! {
            "owner": user_id.to_owned(),
        },
        None
    )
    .await.unwrap();
    if expenses_found == None {
        expense.id = 0;
        let mut expense_list = ExpenseList::new(expense, user_id);
        expense_list.id_count += 1;
        let serialized_expense = bson::to_bson(&expense_list).unwrap();
        let document = serialized_expense.as_document().unwrap();
        let insert_result = expenses.insert_one(document.to_owned(), None).await;
        if insert_result.is_err() {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    else {
        let mut expense_list: ExpenseList = bson::from_bson(mongodb::bson::Bson::Document(expenses_found.unwrap())).unwrap();
        
        expense.id = expense_list.id_count;
        expense_list.id_count += 1;
        expense_list.list.push(expense);
        let serialized_expense = bson::to_bson(&expense_list).unwrap();
        let document = serialized_expense.as_document().unwrap();
        let replace_result = expenses.replace_one(
            doc! {
                "_id": &expense_list.id
            },
            document,
            None,
        ).await;
        if replace_result.is_err() {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }

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
    expense_id: usize,
    user_id: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let expenses = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("expenses");
    let expenses_found = expenses.find_one(
        doc! {
            "owner": user_id.to_owned(),
        },
        None
    )
    .await.unwrap();
    if expenses_found != None {
        let mut expense_list: ExpenseList = bson::from_bson(mongodb::bson::Bson::Document(expenses_found.unwrap())).unwrap();
        
        // expense_list.list.retain(|exp| exp.id != expense_id);
        for mut expense in expense_list.to_owned().list {
            if expense.id == expense_id {
                expense.deleted = true;
                break;
            }
        }
        let serialized_expense = bson::to_bson(&expense_list).unwrap();
        let document = serialized_expense.as_document().unwrap();
        let replace_result = expenses.replace_one(
            doc! {
                "_id": &expense_list.id
            },
            document,
            None,
        ).await;
        if replace_result.is_err() {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }

    }
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

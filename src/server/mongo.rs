use mongodb::{
    bson::{oid::ObjectId, Document},
    options::{ClientOptions, ResolverConfig},
    Client, results::InsertOneResult,
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fmt};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub password_hashed: String,
    // pub salt: String,      
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Expense {
    pub cost: f64,
    pub name: String,
    pub id: usize,
    pub deleted: bool,
} 

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpenseList {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub list: Vec<Expense>,
    pub owner: String,
    pub id_count: usize,
}
impl ExpenseList {
    pub fn new(expense: Expense, owner: String) -> Self {
        let expenses = vec![expense];
        Self {
            id: None,
            list: expenses,
            owner: owner,
            id_count: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub expense_list: Vec<Expense>,
    pub owner: String,
    pub max_value: f64,
    pub curr_value: f64,
    pub name: String,
}



#[derive(Debug, Clone)]
pub struct Db {
    pub client: Option<Client>,
}

impl Db {
    pub fn new() -> Self {
        Db { client: None }
    }
    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        self.client = Some(Client::with_options(options)?);
        println!("Databases:");
        for name in self
            .client
            .as_mut()
            .unwrap()
            .list_database_names(None, None)
            .await?
        {
            println!("- {}", name);
        }

        Ok(())
    }
    pub async fn find_one(&mut self, database: &str, collection: &str, filter: Document) -> Option<Document> {
        let users = self
            .client
            .as_ref()
            .unwrap()
            .database(database)
            .collection::<Document>(collection);
        users.find_one(filter, None,).await.unwrap()
    }
    pub async fn insert_one(&mut self, database: &str, collection: &str, document: &Document) -> mongodb::error::Result<InsertOneResult> {
        let users = self
            .client
            .as_ref()
            .unwrap()
            .database(database)
            .collection::<Document>(collection);
        users.insert_one(document.to_owned(), None).await
    }
}

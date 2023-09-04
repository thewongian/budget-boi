use mongodb::{
    bson::oid::ObjectId,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub password_hashed: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Expense {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub cost: f64,
    pub name: String,
    pub owner: String,
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
}

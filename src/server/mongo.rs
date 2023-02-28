use dotenv::dotenv;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password_hashed: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Expense {
    pub id: u64,
    pub cost: f64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Db {
    pub client: Option<Client>,
}


impl Db {
    pub fn new() -> Self {
        Db {
            client: None
        }
    }
    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        self.client = Some(Client::with_options(options)?);
        println!("Databases:");
        for name in self.client.as_mut().unwrap().list_database_names(None, None).await? {
            println!("- {}", name);
        }

        Ok(())
    }

}

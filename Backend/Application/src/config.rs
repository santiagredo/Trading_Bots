use std::{env, str::FromStr};

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::environments::Environments;

#[derive(Debug, Clone)]
pub struct Config {
    pub db: DatabaseConnection,
    pub api_key: String,
    pub secret_pass: String,
    pub environment: Environments,
    pub url: String,
    pub port: String,
}

fn get_env_val<T: FromStr>(key: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .expect(&format!("{key} not set"))
        .parse::<T>()
        .expect(&format!("Can't parse {key} to desired type"))
}

pub async fn load_settings() {
    dotenvy::dotenv().unwrap();

    Migrator::up(&get_config().await.db, None).await.unwrap();
}

pub async fn get_config() -> Config {
    let env_db_url: String = get_env_val("DATABASE_URL");
    let db = Database::connect(&env_db_url).await.unwrap();

    let api_key: String = get_env_val("API_KEY");
    let secret_pass: String = get_env_val("SECRET_PASS");

    let environment_key = get_env_val::<String>("ENVIRONMENT");
    let environment = match environment_key.as_ref() {
        "PRO" => Environments::PRO,
        "UAT" => Environments::UAT,
        "STG" => Environments::STG,
        _ => Environments::DEV,
    };

    let url = get_env_val::<String>("URL");
    let port = get_env_val::<String>("PORT");

    Config {
        db,
        api_key,
        secret_pass,
        environment,
        url,
        port,
    }
}

// pub async fn get_bt_config() -> Config {
//     let db_name = format!("tradinb_bots_bt_{}", Local::now().to_string());
//     let master_conn_str = "postgres://postgres:password@localhost/postgres";

//     let master_db = Database::connect(master_conn_str).await.unwrap();
//     master_db
//         .execute_unprepared(&format!("CREATE DATABASE {};", db_name))
//         .await
//         .unwrap();

//     let db = Database::connect(db_name).await.unwrap();
//     Migrator::up(&db, None).await.unwrap();

//     Config { db }
// }

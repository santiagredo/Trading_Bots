use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use models::structs::Configuration;
use tracing::error_span;

use crate::static_strings::{DEV_DATABASE_URL, PROD_DATABASE_URL};
use crate::utils::Response;
use crate::{handler::Configurations, utils::Data};

impl Configurations<Data> {
    pub fn insert_configuration_data(self) -> Result<Configuration, Response> {
        let mut response = Response {
            code: 500,
            message: String::new(),
        };

        let mut content = String::new();

        {
            let mut file = match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(".env")
            {
                Err(err) => {
                    error_span!(
                        "Error - Env",
                        error = %err,
                        file_path = file!().to_string(),
                        line_number = line!().to_string(),
                    )
                    .in_scope(|| {
                        tracing::error!("Failed to read/create/open env file");
                    });

                    response.message = format!("Failed to read/create/open env file");

                    return Err(response);
                }
                Ok(val) => val,
            };

            if let Err(err) = file.read_to_string(&mut content) {
                error_span!(
                    "Error - Env",
                    error = %err,
                    file_path = file!().to_string(),
                    line_number = line!().to_string(),
                )
                .in_scope(|| {
                    tracing::error!("Failed to append bytes to buffer");
                });

                response.message = format!("Failed to append bytes to buffer");

                return Err(response);
            };
        }

        let mut content_map: HashMap<String, String> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();

                if line.is_empty() || line.starts_with('#') {
                    return None;
                }

                line.split_once('=').map(|(key, value)| {
                    (
                        key.trim().to_string(),
                        value.trim().to_string(),
                    )
                })
            })
            .collect();

        if let Some(api_key) = self.model.api_key {
            content_map.insert("api_key".to_owned(), api_key);
        }

        if let Some(secret_pass) = self.model.secret_pass {
            content_map.insert("secret_pass".to_owned(), secret_pass);
        }

        let mut file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(".env")
        {
            Err(err) => {
                error_span!(
                    "Error - Env",
                    error = %err,
                    file_path = file!().to_string(),
                    line_number = line!().to_string(),
                )
                .in_scope(|| {
                    tracing::error!("Failed to write/create/trunce env file");
                });

                response.message = format!("Failed to write/create/trunce env file");

                return Err(response);
            }
            Ok(val) => val,
        };

        for (key, value) in &content_map {
            if let Err(err) = writeln!(file, "{}={}", key, value) {
                error_span!(
                    "Error - Env",
                    error = %err,
                    file_path = file!().to_string(),
                    line_number = line!().to_string(),
                    env_key = %key,
                    env_value = %value,
                )
                .in_scope(|| {
                    tracing::error!("Failed to write to env file");
                });
                response.message = format!("Failed to write to env file: {key}");

                return Err(response);
            };
        }

        let config = Configuration {
            dev_database_url: DEV_DATABASE_URL.to_owned(),
            prod_database_url: PROD_DATABASE_URL.to_owned(),
            api_key: content_map.get("api_key").cloned().unwrap_or_default(),
            secret_pass: content_map.get("secret_pass").cloned().unwrap_or_default(),
        };

        Ok(config)
    }

    pub fn select_configuration_data() -> Configuration {
        let mut content = String::new();
        let mut map = HashMap::new();

        let cwd = std::env::current_dir().unwrap_or_default();
        let path = cwd.join(".env");

        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
        {
            Err(err) => {
                error_span!(
                    "Error - Env",
                    error = %err,
                    file_path = file!().to_string(),
                    line_number = line!().to_string(),
                )
                .in_scope(|| {
                    tracing::error!("Failed to select env file");
                });

                return Configuration::default();
            }
            Ok(val) => val,
        };

        if let Err(err) = file.read_to_string(&mut content) {
            error_span!(
                "Error - Env",
                error = %err,
                file_path = file!().to_string(),
                line_number = line!().to_string(),
            )
            .in_scope(|| {
                tracing::error!("Failed to append bytes to buffer");
            });

            return Configuration::default();
        }

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        for line in lines {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let mut configuration = Configuration {
            dev_database_url: DEV_DATABASE_URL.to_owned(),
            prod_database_url: PROD_DATABASE_URL.to_owned(),
            ..Default::default()
        };

        if let Some(api_key) = map.get("api_key") {
            configuration.api_key = api_key.to_string();
        }

        if let Some(secret_pass) = map.get("secret_pass") {
            configuration.secret_pass = secret_pass.to_string();
        }

        configuration
    }
}

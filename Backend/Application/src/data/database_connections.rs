use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::error_span;

use crate::{handler::DBC, utils::Response};

impl DBC {
    pub async fn set_database_data(
        connection_string: &str,
    ) -> Result<DatabaseConnection, Response> {
        let conn = match Database::connect(connection_string).await {
            Err(err) => {
                error_span!(
                    "Error - Database Connection",
                    error = %err,
                    file_path = file!().to_string(),
                    line_number = line!().to_string(),
                )
                .in_scope(|| {
                    tracing::error!("Failed to setup database connection");
                });

                return Err(Response {
                    code: 500,
                    message: format!("Failed to setup database connection"),
                });
            }
            Ok(val) => val,
        };

        if let Err(err) = Migrator::up(&conn, None).await {
            error_span!(
                "Error - Database Connection",
                error = %err,
                file_path = file!().to_string(),
                line_number = line!().to_string(),
            )
            .in_scope(|| {
                tracing::error!("Failed to run database migration");
            });

            return Err(Response {
                code: 500,
                message: format!("Failed to run database migration"),
            });
        }

        Ok(conn)
    }
}

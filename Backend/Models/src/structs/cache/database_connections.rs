use sea_orm::DbConn;

#[derive(Clone)]
pub struct DatabaseManager {
    pub dev: DbConn,
    pub prod: DbConn,
}

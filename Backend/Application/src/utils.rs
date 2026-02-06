use crate::handler::DBC;
use actix_web::{http::StatusCode, HttpResponse};
use migration::async_trait::async_trait;
use models::enums::LifecycleState;
use models::structs::{Environments, QueryOptions};
use sea_orm::{DatabaseConnection, DbErr, SqlErr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub struct Utils;

impl Utils {
    pub fn validate_empty_field(field: String, field_name: &str) -> Result<String, String> {
        match field.trim().is_empty() {
            true => Err(format!("{field_name} cannot be empty")),
            false => Ok(field.trim().to_lowercase()),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conditions {
    pub get_all: bool,
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
}

impl Response {
    pub fn bad_request(item: String) -> Response {
        Response {
            code: 400,
            message: format!("Bad request: {item}"),
        }
    }

    pub fn not_found(item: String) -> Response {
        Response {
            code: 404,
            message: format!("{item} not found"),
        }
    }

    pub fn server_error(item: String) -> Response {
        Response {
            code: 500,
            message: format!("Server error: {item}"),
        }
    }
}

pub fn error_response(response: Response) -> HttpResponse {
    HttpResponse::build(
        StatusCode::from_u16(response.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    )
    .json(Response {
        code: response.code,
        message: response.message,
    })
}

pub fn handle_db_error(err: &DbErr) -> Response {
    if let Some(sql_err) = err.sql_err() {
        match sql_err {
            SqlErr::UniqueConstraintViolation(msg) => Response {
                code: 400,
                message: format!("Unique constraint violation: {msg}"),
            },
            SqlErr::ForeignKeyConstraintViolation(msg) => Response {
                code: 400,
                message: format!("Foreign key constraint violation: {msg}"),
            },
            _ => Response {
                code: 500,
                message: format!("Unexpected SQL error"),
            },
        }
    } else {
        Response {
            code: 500,
            message: format!("Database error"),
        }
    }
}

pub fn handle_user_err(err: String) -> Response {
    Response {
        code: 400,
        message: err,
    }
}

#[macro_export]
macro_rules! log_trait_db_error {
    ($err:expr, $req:expr) => {{
        let error_log_request = models::structs::ErrorLogRequest {
            file_path: Some(file!().to_string()),
            line_number: Some(line!().to_string()),
            function_name: Some(function_name!().to_string()),
            request: Some(serde_json::to_string(&$req).unwrap_or_default()),
            error_type: Some(format!("{:?}", $err)),
            error_details: Some($err.to_string()),
            ..Default::default()
        };

        error_log_request
    }};
}

pub fn trim_option(value: &mut Option<String>) {
    if let Some(v) = value {
        *v = v.trim().to_owned();
    }
}

#[derive(Debug, Clone)]
pub struct DbRepo {
    pub data: DatabaseConnection,
    pub environment: Environments,
}

impl DbRepo {
    pub async fn new(environment: Environments) -> Result<Self, Response> {
        let data = DBC::db(&environment).await?;

        Ok(Self { data, environment })
    }
}

#[derive(Debug, Clone)]
pub struct MockRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub data: Arc<Mutex<Vec<T>>>,
}

impl<T> MockRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
pub trait Insert<RE, MO> {
    async fn insert(&self, req: RE) -> Result<MO, Response>;
}

#[async_trait]
pub trait Select<RE, MO> {
    async fn select(&self, req: RE) -> Result<Option<MO>, Response>;
    async fn select_many(&self, req: RE, query: Option<QueryOptions>) -> Result<Vec<MO>, Response>;
}

#[async_trait]
pub trait Update<RE, MO> {
    async fn update(&self, req: RE) -> Result<MO, Response>;
}

#[async_trait]
pub trait Delete<RE> {
    async fn delete(&self, req: RE) -> Result<u64, Response>;
}

pub trait Repository<RE, MO>:
    Insert<RE, MO> + Select<RE, MO> + Update<RE, MO> + Delete<RE>
{
}

impl<RE, MO, T> Repository<RE, MO> for T where
    T: Insert<RE, MO> + Select<RE, MO> + Update<RE, MO> + Delete<RE>
{
}

#[async_trait]
pub trait EntityCache<E>: Send + Sync
where
    E: Eq + Hash + Send + Sync,
{
    type Key: Eq + Hash + Send + Sync;
    type Value: Clone + Send + Sync;
    type Collection: Clone + Send + Sync;

    async fn state(&self, env: E) -> LifecycleState;
    async fn set_state(&self, env: E, state: LifecycleState) -> Result<(), String>;

    async fn get_all(&self, env: E) -> Option<Self::Collection>;
    async fn get(&self, env: E, key: Self::Key) -> Option<Self::Value>;

    async fn set_all(&self, env: E, values: Vec<Self::Value>) -> Result<(), String>;

    async fn upsert(&self, env: E, key: Self::Key, value: Self::Value) -> Result<(), String>;

    async fn remove(&self, env: E, key: Self::Key) -> Result<Option<Self::Value>, String>;
    async fn remove_all(&self, env: E) -> Result<HashMap<Self::Key, Self::Value>, String>;
    async fn reset(&self, env: E) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub enum RepoBackend {
    Db(DbRepo),
    Mock,
}

#[derive(Debug, Clone)]
pub struct AnyRepo<RE, MO> {
    backend: RepoBackend,
    _phantom: PhantomData<(RE, MO)>,
}

impl<RE, MO> AnyRepo<RE, MO>
where
    RE: Send + Sync + 'static,
    MO: Clone + Send + Sync + 'static,
{
    pub fn new(backend: RepoBackend) -> Self {
        Self {
            backend,
            _phantom: PhantomData,
        }
    }

    pub fn is_mock(&self) -> bool {
        matches!(self.backend, RepoBackend::Mock)
    }

    pub fn environment(&self) -> Option<Environments> {
        match &self.backend {
            RepoBackend::Db(repo) => Some(repo.environment),
            RepoBackend::Mock => None,
        }
    }
}

#[async_trait]
impl<RE, MO> Insert<RE, MO> for AnyRepo<RE, MO>
where
    RE: Send + Sync + 'static,
    MO: Clone + Send + Sync + 'static,
    DbRepo: Insert<RE, MO>,
    MockRepo<MO>: Insert<RE, MO>,
{
    async fn insert(&self, req: RE) -> Result<MO, Response> {
        match &self.backend {
            RepoBackend::Db(repo) => repo.insert(req).await,
            RepoBackend::Mock => MockRepo::<MO>::new().insert(req).await,
        }
    }
}

#[async_trait]
impl<RE, MO> Select<RE, MO> for AnyRepo<RE, MO>
where
    RE: Send + Sync + 'static,
    MO: Clone + Send + Sync + 'static,
    DbRepo: Select<RE, MO>,
    MockRepo<MO>: Select<RE, MO>,
{
    async fn select(&self, req: RE) -> Result<Option<MO>, Response> {
        match &self.backend {
            RepoBackend::Db(repo) => repo.select(req).await,
            RepoBackend::Mock => MockRepo::<MO>::new().select(req).await,
        }
    }

    async fn select_many(&self, req: RE, query: Option<QueryOptions>) -> Result<Vec<MO>, Response> {
        match &self.backend {
            RepoBackend::Db(repo) => repo.select_many(req, query).await,
            RepoBackend::Mock => MockRepo::<MO>::new().select_many(req, query).await,
        }
    }
}

#[async_trait]
impl<RE, MO> Update<RE, MO> for AnyRepo<RE, MO>
where
    RE: Send + Sync + 'static,
    MO: Clone + Send + Sync + 'static,
    DbRepo: Update<RE, MO>,
    MockRepo<MO>: Update<RE, MO>,
{
    async fn update(&self, req: RE) -> Result<MO, Response> {
        match &self.backend {
            RepoBackend::Db(repo) => repo.update(req).await,
            RepoBackend::Mock => MockRepo::<MO>::new().update(req).await,
        }
    }
}

#[async_trait]
impl<RE, MO> Delete<RE> for AnyRepo<RE, MO>
where
    RE: Send + Sync + 'static,
    MO: Clone + Send + Sync + 'static,
    DbRepo: Delete<RE>,
    MockRepo<MO>: Delete<RE>,
{
    async fn delete(&self, req: RE) -> Result<u64, Response> {
        match &self.backend {
            RepoBackend::Db(repo) => repo.delete(req).await,
            RepoBackend::Mock => MockRepo::<MO>::new().delete(req).await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepoFactory {
    backend: RepoBackend,
}

impl RepoFactory {
    pub async fn db(environment: Environments) -> Result<Self, Response> {
        Ok(Self {
            backend: RepoBackend::Db(DbRepo::new(environment).await?),
        })
    }

    pub fn mock() -> Self {
        Self {
            backend: RepoBackend::Mock,
        }
    }

    pub fn repo<RE, MO>(&self) -> AnyRepo<RE, MO>
    where
        RE: Send + Sync + 'static,
        MO: Clone + Send + Sync + 'static,
    {
        AnyRepo::new(self.backend.clone())
    }
}

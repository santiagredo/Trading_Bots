use crate::structs::Environments;

pub trait IntoEnvRequest<T> {
    fn into_env_request(self) -> (Environments, T);
}

use models::entities::tasks::Model;

use crate::{
    handler::{Tasks, DBC},
    utils::{handle_user_err, Cache, Core, Response},
};

impl Tasks<Core> {
    // db
    pub async fn select_tasks_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_tasks_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_task_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .update_task_logic()
            .map_err(handle_user_err)?
            .next_phase()
            .update_task_data(&DBC::db(&env).await?)
            .await
    }

    // memory
    pub async fn get_active_tasks_core(self) -> Option<Vec<Model>> {
        let env = self.environment;
        Tasks::get_active_tasks_cache(&env).await
    }

    pub async fn start_active_tasks_core(self) -> Result<(), Response> {
        let environment = self.environment;

        if !Tasks::get_active_tasks_status_cache(&environment).await {
            let mut tasks_request = Tasks::default().with_env(environment);
            tasks_request.model.is_active = Some(true);

            let tasks = tasks_request.select_tasks().await?;

            Tasks::set_active_tasks_cache(&environment, tasks).await;
        }

        Ok(())
    }

    pub async fn stop_active_tasks_core(self) {
        let environment = &self.environment;

        Tasks::<Cache>::stop_active_tasks_cache(environment).await
    }
}

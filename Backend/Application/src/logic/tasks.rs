use crate::{handler::Tasks, utils::Logic};

impl Tasks<Logic> {
    pub fn update_task_logic(mut self) -> Result<Self, String> {
        if self.model.id.is_some_and(|id| id <= 0) {
            return Err(format!("Invalid task ID: {:?}", self.model.id));
        }

        if self.model.cooldown.is_some_and(|cooldown| cooldown <= 300) {
            self.model.cooldown = Some(300)
        }

        Ok(self)
    }
}

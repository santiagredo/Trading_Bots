use models::structs::TaskRequest;

pub fn update_task_logic(task: &mut TaskRequest) -> Result<(), String> {
    if task.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid task ID: {:?}", task.id));
    }

    if task.cooldown.is_none_or(|cooldown| cooldown <= 300) {
        task.cooldown = Some(300)
    }

    Ok(())
}

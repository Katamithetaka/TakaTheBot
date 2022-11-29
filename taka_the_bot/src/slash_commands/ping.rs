use async_trait::async_trait;


pub struct Command {}

#[async_trait]
impl super::Command for Command {
    async fn execute(&self) {}

    fn get_name(&self) -> String {
        "ping".to_string()
    }
}
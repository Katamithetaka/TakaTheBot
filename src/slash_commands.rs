
use async_trait::async_trait;

use std::{collections::HashMap, sync::Arc};


#[async_trait]
pub trait Command {
    async fn execute(&self);
    fn get_name(&self) -> String;
}




pub type CommandType = Arc<dyn Command>;
pub type CommandCollectionType = std::collections::HashMap<String, CommandType>;

macro_rules! make_command {
    ($collection: ident, $command: ident) => { 
        {
            let command = $command::Command{};
            $collection.insert(command.get_name(), Arc::new(command));
        }
    }
}

mod ping;

pub fn get_commands() -> CommandCollectionType {
    let mut collection = CommandCollectionType::new();

	make_command!(collection, ping);


    collection
}

use async_trait::async_trait;

#[allow(unused_imports)]
use std::{collections::HashMap, sync::Arc};

#[async_trait]
pub trait Command {
    async fn execute(&self);
    fn get_name(&self) -> String;
}

pub type CommandType = Arc<dyn Command>;
pub type CommandCollectionType = HashMap<String, CommandType>;

#[allow(unused_macros)]
macro_rules! make_command {
    ($collection: ident, $command: ident) => {{
        let command = $command::Command {};
        $collection.insert(command.get_name(), Arc::new(command));
    }};
}

use automatic_generation::make_get_slash_commands;

make_get_slash_commands!("./taka_the_bot/src/slash_commands");

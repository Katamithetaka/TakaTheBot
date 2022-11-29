use crate::get_all_filenames_of_ext_in_folder;
use proc_macro::TokenStream;

macro_rules! GET_COMMANDS_TEMPLATE {
    ($module_str: ident, $make_command_str: ident) => {
        format!(r#"
        {}

        pub fn get_commands() -> CommandCollectionType {{
            let mut collection = CommandCollectionType::new();
            
            {}
            
            collection
        }}
        "#, $module_str, $make_command_str)
    }
}

macro_rules! MODULE_IMPORT {
    ($name: ident) => {
        &format!("mod {};\n", $name)
    };
}

macro_rules! MAKE_SLASH_COMMAND {
    ($name: ident) => {
        &format!("make_command!(collection, {});\n", $name)
    };
}

pub(crate) fn make_get_slash_commands(_item: TokenStream) -> TokenStream {
    let dir: String = String::from_utf8(
        _item
            .to_string()
            .into_bytes()
            .into_iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .collect(),
    )
    .unwrap_or("taka_the_bot/src/slash_commands".to_owned());

    let slash_commands = get_all_filenames_of_ext_in_folder(&dir, Some("rs"));
    let mut slash_command_modules = String::new();
    let mut slash_command_make_commands = String::new();

    for command in slash_commands {
        slash_command_modules = slash_command_modules + MODULE_IMPORT!(command);
        slash_command_make_commands = slash_command_make_commands + MAKE_SLASH_COMMAND!(command);
    }
    GET_COMMANDS_TEMPLATE!(slash_command_modules, slash_command_make_commands)
        .as_str()
        .parse()
        .unwrap()
}

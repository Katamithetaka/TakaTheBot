extern crate proc_macro;
use proc_macro::TokenStream;

use std::fs::{self, DirEntry};

fn is_file(entry: &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    } else {
        false
    }
}

fn get_filename_and_extension(
    file: &Result<DirEntry, std::io::Error>,
) -> (Option<String>, Option<String>) {
    if let Ok(entry) = file {
        if is_file(&entry) {
            let file_name = entry.file_name();
            let file_name = if let Some(file_name) = file_name.into_string().ok() {
                file_name
            } else {
                return (None, None);
            };
            let split: Vec<_> = file_name.split(".").collect();
            let (file_name, ext) = (split.get(0), split.get(1));

            let file_name = if let Some(file_name) = file_name {
                file_name.clone().to_string()
            } else {
                return (None, None);
            };

            let ext = if let Some(ext) = ext {
                ext.clone().to_string()
            } else {
                return (Some(file_name), None);
            };

            return (Some(file_name), Some(ext));
        }
    }

    (None, None)
}

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

#[proc_macro]
pub fn make_get_slash_commands(_item: TokenStream) -> TokenStream {
    let mut slash_commands = vec![];
    let _str: String = String::from_utf8(
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

    if let Ok(result) = fs::read_dir(_str) {
        for dir_entry in result {
            let (file_name, ext) = get_filename_and_extension(&dir_entry);

            let file_name = if let Some(file_name) = file_name {
                file_name
            } else {
                continue;
            };

            let ext = if let Some(ext) = ext {
                ext
            } else {
                continue;
            };

            if ext != "rs" {
                continue;
            }

            slash_commands.push(file_name);
        }
    }

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

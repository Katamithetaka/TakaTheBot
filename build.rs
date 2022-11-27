use std::{fs::{self, File, DirEntry}, io::{BufWriter, Write}};

pub fn is_file(entry: &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    }
    else {
        false
    }
}

pub fn get_file<'a>(file: &'a Result<DirEntry, std::io::Error>) -> (Option<&'a DirEntry>, Option<File>) {
    if let Ok(entry) = file {
        if is_file(&entry) {
            return (Some(entry), fs::File::open(entry.path()).ok())
        }
        else {
            return (Some(entry), None)
        }
    }

    (None, None)
}

pub fn get_filename_and_extension(file: &Result<DirEntry, std::io::Error>) -> (Option<String>, Option<String>) {
    if let Ok(entry) = file {
        if is_file(&entry) { 
            let file_name = entry.file_name();
            let file_name = if let Some(file_name) = file_name.into_string().ok() {
                file_name
            } else {
                return (None, None)
            };
            let split:Vec<_> = file_name.split(".").collect();
            let (file_name, ext) = (split.get(0), split.get(1));

            let file_name = if let Some(file_name) = file_name {
                file_name.clone().to_string()
            } else {
                return (None, None)
            };

            let ext = if let Some(ext) = ext {
                ext.clone().to_string()
            } else {
                return (Some(file_name), None)
            };

            return (Some(file_name), Some(ext));
        }
    }

    (None, None)
}

macro_rules! SLASH_COMMANDS_FILE{
    ($modules: ident, $make_commands: ident) => { format!(r#"
use async_trait::async_trait;

use std::{{collections::HashMap, sync::Arc}};


#[async_trait]
pub trait Command {{
    async fn execute(&self);
    fn get_name(&self) -> String;
}}




pub type CommandType = Arc<dyn Command>;
pub type CommandCollectionType = std::collections::HashMap<String, CommandType>;

macro_rules! make_command {{
    ($collection: ident, $command: ident) => {{ 
        {{
            let command = $command::Command{{}};
            $collection.insert(command.get_name(), Arc::new(command));
        }}
    }}
}}

{}
pub fn get_commands() -> CommandCollectionType {{
    let mut collection = CommandCollectionType::new();

{}

    collection
}}
"#, $modules, $make_commands)
    };
}

macro_rules! MODULE_IMPORT {
    ($arg: ident) => {
        &format!("mod {};\n", $arg)
    };
}

macro_rules! MAKE_SLASH_COMMAND {
    ($arg: ident) => {
        &format!("\tmake_command!(collection, {});\n", $arg)
    };
}

fn main() 
{
    
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/slash_commands/*.rs");
    let mut slash_commands = vec![];
    if let Ok(result) = fs::read_dir("src/slash_commands/") {
        for dir_entry in result {
            let (file_name, ext) = get_filename_and_extension(&dir_entry);

            let file_name = if let Some(file_name) = file_name {
                file_name
            }
            else {
                continue;
            };

            let ext = if let Some(ext) = ext {
                ext
            }
            else {
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

    let slash_commands_file = SLASH_COMMANDS_FILE!(slash_command_modules, slash_command_make_commands);

    let file = fs::File::create("src/slash_commands.rs").unwrap();
    let mut write_buf = BufWriter::new(file);

    write_buf.write(slash_commands_file.as_bytes()).unwrap();
}
extern crate proc_macro;

mod slash_commands;
use std::fs::DirEntry;

use proc_macro::TokenStream;

pub(crate) fn is_file(entry: &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    } else {
        false
    }
}

pub(crate) fn get_filename_and_extension(
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

pub(crate) fn get_all_filenames_of_ext_in_folder(
    dir: &str,
    extension: Option<&str>,
) -> Vec<String> {
    let mut array = vec![];
    if let Ok(result) = std::fs::read_dir(dir) {
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
            if let Some(e) = extension {
                if ext != e {
                    continue;
                }
            }

            array.push(file_name);
        }
    }

    array
}

#[proc_macro]
pub fn make_get_slash_commands(item: TokenStream) -> TokenStream {
    slash_commands::make_get_slash_commands(item)
}

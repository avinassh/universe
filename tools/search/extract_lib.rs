use search_proto_rust::*;

use std::path::{Path, PathBuf};

pub fn extract_code(root_dir: &Path, output_filename: &str) {
    let f = std::fs::File::create(output_filename).unwrap();
    let mut w = std::io::BufWriter::new(f);
    let mut builder = recordio::RecordIOWriterOwned::new(Box::new(w));

    let prefix = root_dir.to_owned().into_os_string().into_string().unwrap();

    let mut prefix_len = prefix.len();
    if !prefix.ends_with("/") {
        prefix_len += 1;
    }

    extract_from_dir(prefix_len, &root_dir.to_str().unwrap(), &mut builder);
}

fn extract_from_dir(
    prefix: usize,
    root_dir: &str,
    output: &mut recordio::RecordIOWriterOwned<File>,
) {
    let mut children = std::collections::BTreeMap::new();
    for result in std::fs::read_dir(root_dir).unwrap() {
        let result = result.unwrap();
        children.insert(
            result.path().into_os_string().into_string().unwrap(),
            result.file_type().unwrap(),
        );
    }

    let mut directories = Vec::new();

    for (path, filetype) in children {
        let mut f = File::new();
        if filetype.is_dir() {
            f.set_is_directory(true);
            directories.push(path.clone());
        } else {
            let contents = match std::fs::read_to_string(&path) {
                Ok(s) => s,
                Err(_) => {
                    // If it fails to read as a string, it must be a binary file
                    f.set_is_binary(true);
                    String::new()
                }
            };
            f.set_content(contents);
        }

        f.set_filename(path[prefix..].to_owned());
        let filename = f.get_filename().to_owned();
        let is_dir = f.get_is_directory();

        let depth = filename.matches("/").count();
        output.write(&f);
    }

    for dir in directories {
        extract_from_dir(prefix, &dir, output);
    }
}

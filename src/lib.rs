use goblin::{self, Object};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(paths: &[&str]) {
    let blob_paths = find_blobs_in_paths(&paths);
    let blobs_to_dependencies = get_dependencies(&blob_paths);
    let missing_blobs = identify_missing(&blobs_to_dependencies);
    display_missing_blobs(&missing_blobs);
}

fn find_blobs_in_paths(paths: &[&str]) -> Vec<PathBuf> {
    let dirs = paths
        .iter()
        .map(Path::new)
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    let blob_paths: Vec<PathBuf> = dirs
        .iter()
        .map(|dir| fs::read_dir(dir).expect("Could not read directory."))
        .flat_map(|read_dir| {
            read_dir.map(|dir_entry| dir_entry.expect("Could not read directory entry.").path())
        })
        .filter(|path| match path.extension() {
            // Assume that valid blobs have ".so" extension.
            Some(ext) => ext == "so",
            None => false,
        })
        .collect();

    blob_paths
}

fn get_dependencies(blob_paths: &Vec<PathBuf>) -> HashMap<String, Vec<String>> {
    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

    blob_paths.iter().for_each(|path| {
        let filename = path
            .file_name()
            .expect("Could not get file name.")
            .to_str()
            .expect("Could not convert to string.")
            .to_owned();

        let buffer = fs::read(&path).expect("Could not read path.");
        let obj = goblin::Object::parse(&buffer);

        if let Ok(Object::Elf(elf)) = obj {
            let deps: Vec<String> = elf.libraries.iter().map(|dep| dep.to_string()).collect();
            dependencies.insert(filename, deps);
        }
    });

    dependencies
}

fn identify_missing(
    blobs_to_dependencies: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut dependencies_to_blobs: HashMap<String, Vec<String>> = HashMap::new();
    blobs_to_dependencies.iter().for_each(|(blob, deps)| {
        deps.iter().for_each(
            |dependency| match dependencies_to_blobs.get_mut(dependency) {
                Some(dependants) => {
                    dependants.push(blob.to_owned());
                }
                None => {
                    dependencies_to_blobs.insert(dependency.to_owned(), vec![blob.to_owned()]);
                }
            },
        )
    });

    let mut missing_blobs: HashMap<String, Vec<String>> = HashMap::new();

    for dep in dependencies_to_blobs.keys() {
        if !blobs_to_dependencies.contains_key(dep) {
            // Dependency is not present.
            let missing_dep = dep.to_owned();
            let blobs_requiring_missing_dep = dependencies_to_blobs[dep].to_owned();
            missing_blobs.insert(missing_dep, blobs_requiring_missing_dep);
        }
    }

    missing_blobs
}

fn display_missing_blobs(missing_blobs: &HashMap<String, Vec<String>>) {
    for blob in missing_blobs.keys() {
        println!("{} required by: {}", blob, missing_blobs[blob].join("; "));
    }
}

use goblin::{self, Object};
use std::collections::HashMap;
use std::path::Path;
use std::{collections::HashSet, fs};

pub fn run(paths: &[String]) -> goblin::error::Result<()> {
    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let mut present: HashSet<String> = HashSet::new();

    let dirs = paths
        .iter()
        .map(Path::new)
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    for dir in dirs.iter() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            let filename = path
                .file_name()
                .expect("Could not get file name.")
                .to_str()
                .expect("Could not convert to string.")
                .to_string();

            present.insert(filename.clone());

            if let Some(ext) = path.extension() {
                if ext != "so" {
                    continue;
                }
            } else {
                continue;
            }

            let buffer = fs::read(&path)?;
            let obj = goblin::Object::parse(&buffer);

            if let Ok(Object::Elf(elf)) = obj {
                for dep_str in elf.libraries {
                    let dep = dep_str.to_string();
                    match dependencies.get_mut(&dep) {
                        Some(dependants) => {
                            dependants.push(filename.clone());
                        }
                        None => {
                            dependencies.insert(dep, vec![filename.clone()]);
                        }
                    }
                }
            }
        }
    }

    let mut missing: Vec<String> = vec![];

    for (dep, _) in &dependencies {
        if !present.contains(dep) {
            missing.push(dep.to_string());
        }
    }

    for m in missing {
        println!("{} required by: {}", m, dependencies[&m].join("; "));
    }

    Ok(())
}

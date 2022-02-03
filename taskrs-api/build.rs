use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

use build_const::ConstWriter;
use glob::glob;
use serde::{Deserialize, Serialize};

fn main() {
    // Rerun if permission files changed
    println!("cargo:rerun-if-changed=permissions/");

    let permissions: Vec<Permission> = glob("permissions/**/*.json")
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|path| {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            serde_json::from_reader::<_, Vec<Permission>>(reader).unwrap()
        })
        .flatten()
        .collect();

    let mut constants = ConstWriter::for_build("permissions")
        .unwrap()
        .finish_dependencies();

    permissions.iter().cloned().for_each(|p| {
        let name = format!("{}_{}", &p.group, &p.name).to_uppercase();
        constants.add_value(&name, "(&str, &str)", (p.group, p.name))
    });

    let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("permissions.json");
    let mut permission_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    let permissions_json = serde_json::to_string(&permissions).unwrap();
    permission_file
        .write_all(permissions_json.as_bytes())
        .unwrap();
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Permission {
    pub name: String,
    pub group: String,
    pub description: Option<String>,
}

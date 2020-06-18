use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

use std::error::Error;

use crate::FnResult;

pub enum SerdeFormat {
    Json,
    MessagePack
}

/// Trait for every object in a tree structure.
pub trait NodeData {
    /// Use serde to sace this object (along with all its children, if present) into a single file
    fn save_to_file(&self, dir_name: &str, file_name: &str, format: &SerdeFormat) -> FnResult<()>;
}

/// Trait for every object in a tree structure that has children, i.e. everything except leaves.
pub trait TreeData {
    /// Save this objects and its children. If file_levels == 0, this delegates to
    /// save_to_file. Else, it creates a directory and calls save_tree for all it's
    /// non-leaf-children (with file_levels - 1) and save_to_file for all it's leaf-children.
    fn save_tree(&self, dir_name: &str, format: &SerdeFormat, file_levels: usize) -> FnResult<()>;
}

impl<T:Serialize> NodeData for T {
    fn save_to_file(&self, dir_name: &str, file_name: &str, format: &SerdeFormat) -> FnResult<()> {
        let serialized_bin = match format {
            SerdeFormat::MessagePack => rmp_serde::to_vec(self).unwrap(),
            SerdeFormat::Json => serde_json::to_vec(self).unwrap(),
        };
        fs::create_dir_all(&dir_name)?;    
        let file_path = format!("{}/{}", dir_name, file_name);
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create file: {}", why),
            Ok(file) => file,
        };
        match file.write_all(&serialized_bin) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(_) => println!("successfully wrote."),
        }
    
        Ok(())
    }
}
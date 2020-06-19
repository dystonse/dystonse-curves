use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::FnResult;

pub enum SerdeFormat {
    Json,
    MessagePack
}

/// Trait for every object in a tree structure.
pub trait NodeData {
    /// Use serde to save this object (along with all its children, if present) into a single file. This function is implemented by a blanket impl.
    fn save_to_file(&self, dir_name: &str, file_name: &str, format: &SerdeFormat) -> FnResult<()>;
    /// Use serde to load an object of this type (along with all its children, if present) from a single file. This function is implemented by a blanket impl.
    fn load_from_file(dir_name: &str, file_name: &str, format: &SerdeFormat) -> FnResult<Box<Self>>;
}

pub trait LeafData {
    /// Get the file extention (without leading dot) for this type, possibly depending on the given format.
    fn get_ext(format: &SerdeFormat) -> &str {
        match format {
            SerdeFormat::Json => "json",
            SerdeFormat::MessagePack => "mpack"
        }
    }
}

/// Trait for every object in a tree structure that has children, i.e. everything except leaves.
pub trait TreeData : Sized {
    /// Save this objects and its children. If Self::NAME is among the supplied leaves, it will be 
    /// saved into a single file. Otherwise, it will create a directory structure for its children,
    /// which might saved as files or more levels of subdirectories.
    fn save_tree(&self, dir_name: &str, own_name: &str, format: &SerdeFormat, leaves: &Vec<&str>) -> FnResult<()>;
    fn load_tree(dir_name: &str, own_name: &str, format: &SerdeFormat, leaves: &Vec<&str>) -> FnResult<Self>;
}

impl<'a, T> NodeData for T
where T: Serialize + DeserializeOwned
{
    fn save_to_file(&self, dir_name: &str, file_name: &str, format: &SerdeFormat) -> FnResult<()> {
        let serialized_bin = match format {
            SerdeFormat::MessagePack => rmp_serde::to_vec(self).unwrap(),
            SerdeFormat::Json => serde_json::to_vec(self).unwrap(),
        };
        fs::create_dir_all(&dir_name)?;
        let file_ext = "exp"; // Self::get_ext(format);
        let file_path = format!("{}/{}.{}", dir_name, file_name, file_ext);
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create file {}: {}", file_path, why),
            Ok(file) => file,
        };
        match file.write_all(&serialized_bin) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(_) => println!("successfully wrote."),
        }
    
        Ok(())
    }

    fn load_from_file(dir_name: &str, file_name: &str, format: &SerdeFormat)  -> FnResult<Box<Self>> {
        let file_ext = "exp"; // Self::get_ext(format);
        let file_path = format!("{}/{}.{}", dir_name, file_name, file_ext);
        
        let mut f = File::open(file_path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        match rmp_serde::from_read_ref::<_, Self>(&buffer) {
            Err(e) => Err(Box::new(e)),
            Ok(object) => Ok(Box::new(object))
        }
    }
}
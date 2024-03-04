use serde_json::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SERDEdata {
    pub data: Vec<serde_json::Value>,
}

impl SERDEdata {
    //   "./assets/data/json/"
    pub fn new(path: &str) -> Self {
        let mut serde_data = SERDEdata { data: Vec::new() };
        let mut item_counter = 0;

        //https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_str().unwrap();
            let f_exten = match entry.path().extension() {
                Some(exten) => exten.to_str().unwrap(),
                None => "lol",
            };

            if (entry.path().is_file()) && (f_exten == "json") {
                //must be file cause could be directory
                //    println!("json data path: {}", entry.path().display());

                let data = fs::read_to_string(path).expect("Unable to read json data from path");

                let mut json_to_parse: serde_json::Value =
                    serde_json::from_str(&data).expect("Unable to parse itemfile from json");

                let mut dataitemarray: &mut Vec<Value> = &mut Vec::new();

                if json_to_parse.is_object() {
                    dataitemarray.push(json_to_parse);
                } else if json_to_parse.is_array() {
                    dataitemarray = json_to_parse.as_array_mut().unwrap();
                } else {
                    panic!("{json_to_parse:#?}");
                }

                let finalarray = dataitemarray;

                for item in finalarray {
                    item_counter += 1;

                    let ritem = item.take();

                    //serde_data contains every single Json Object in CDDA , after getting this we should now organize them by type and index by id
                    // , but note that not all objects have an id, or have ids which are arrays of sub ids

                    serde_data.data.push(ritem);
                }
            }
        }

        serde_data.process_json_inheritance();
        serde_data
    }

    fn process_json_inheritance(&mut self) {
        let mut typeset = HashSet::new();

        let mut typed_valuemap: HashMap<String, Vec<Value>> = HashMap::new();
        let mut indexed_item_map: HashMap<String, Map<String, Value>> = HashMap::new();

        let data_copy = self.data.clone();

        // generate set of all types
        for value in data_copy.clone() {
            let boop = format!("{:#?}", value);
            let obj = &value.as_object().expect(&boop);

            match obj.get("type") {
                Some(id) => {
                    typeset.insert(id.as_str().unwrap().to_string().to_lowercase());
                }

                None => {
                    println!("{obj:#?}");
                    panic!("no mitem typeeeeeeeeeeeee item count ");
                }
            }
        }

        // initialize hashmap of typed value vectors
        for jstype in &typeset {
            typed_valuemap.insert(jstype.to_string(), vec![]);
        }

        //insert values into hashmap
        for value in data_copy.clone() {
            //into object
            let obj = value.as_object().unwrap();

            //get object type
            let object_type = obj
                .get("type")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .to_lowercase();

            // make the type a lowercase String

            //insert the object into a vector of objects of the same type
            typed_valuemap
                .get_mut(&object_type)
                .unwrap()
                .push(value.clone());
        }

        // println!("{:#?}",typed_valuemap);

        let item_types = vec![
            "armor".to_string(),
            "generic".to_string(),
            "gun".to_string(),
            "ammo".to_string(),
            "comestible".to_string(),
            "tool".to_string(),
            "tool_armor".to_string(),
            //      "monster".to_string(),
        ];


        for typed_vector in &typed_valuemap { // start indexed_item_map init
            let typed_vectortype = typed_vector.0.to_string().to_lowercase();
    
            let value_vector = typed_vector.1;
            if item_types.contains(&typed_vectortype) {
            for obj in value_vector {
             
                    //
                 //   println!("{obj:#?}");
    
                    let object = obj.as_object().unwrap();
    
                    match object.get("abstract") {
                        Some(abid) => {
                            indexed_item_map.try_insert(
                                abid.clone().as_str().unwrap().to_string(),
                                object.clone(),
                            ).unwrap();
                        }
                        None => match object.get("id") {
                            Some(abid) => match abid {
                                serde_json::Value::String(id_string) => {
                                    indexed_item_map.try_insert(id_string.clone(), object.clone()).unwrap();
                                }
    
                                serde_json::Value::Array(id_array) => {
                                    panic!("should handle id array better");
                                    for aid in id_array {
                                        indexed_item_map.try_insert(
                                            aid.clone().as_str().unwrap().to_string(),
                                            object.clone(),
                                        ).unwrap();
                                    }
                                }
    
                                _ => panic!("no suitable value for id"),
                            },
    
                            None => panic!("no suitable value for id") //no id or abstract
                        },
                    }
                }
            }
        } // end indexed_item_map init

       
    }
}

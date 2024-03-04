use serde_json::*;
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
        let mut item_map: Vec<serde_json::Value> = Vec::new(); //<"itemtype",<"id",item>>
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

        serde_data
    }
}

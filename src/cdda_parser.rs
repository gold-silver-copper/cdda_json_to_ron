use serde_json::*;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::collections::{HashSet,HashMap};

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




    fn process_json_inheritance(&mut self){

        let mut typeset = HashSet::new();

        let mut typed_valuemap: HashMap<String, Vec<Value>> = HashMap::new();


        let data_copy = self.data.clone();
        

        for value in data_copy.clone() {
            let boop = format!("{:#?}",value);
            let obj = &value.as_object().expect(&boop);
    
            match obj.get("type") {
                Some(id) => {
                    typeset.insert(id.as_str().unwrap().to_string().to_lowercase());
                }
    
                None => {
                    println!("{obj:#?}");
                    panic!("no mitem typeeeeeeeeeeeee item count ");
                }
            } //end tagging match ritem.get("type"){
        }

        for jstype in &typeset {
            typed_valuemap.insert(jstype.to_string(), vec![]);
            //    println!("{jstype:#?}");
        }

        for value in data_copy.clone() {

            //into object
            let obj = value.as_object().unwrap();
    
            //get object type
            let object_type = obj.get("type").unwrap().as_str().unwrap().to_string().to_lowercase();


            // make the type a lowercase String
        

            //insert the object into a vector of objects of the same type
            typed_valuemap.get_mut(&object_type).unwrap().push(value.clone());
        }









    }




}

use cdda_json_to_ron::CDDAParser;

use serde_json::*;
use std::fs;
use walkdir::WalkDir;
use ron::*;
use serde_transcode::*;
use std::io;
use ron::ser::PrettyConfig;

#[derive(Debug)]
pub struct SERDEdata {
    pub data: Vec<serde_json::Value>,
}

fn main() {
    let mut item_map: Vec<serde_json::Value> = Vec::new(); //<"itemtype",<"id",item>>
    let mut serde_data = SERDEdata { data: Vec::new() };
    let mut item_counter = 0;

    //https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    for entry in WalkDir::new("./assets/data/json/")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_str().unwrap();
        let f_exten = match entry.path().extension() {
            Some(exten) => exten.to_str().unwrap(),
            None => "lol",
        };

let my_config = PrettyConfig::new()
.depth_limit(4)
// definitely superior (okay, just joking)
.indentor("\t".to_owned());

        if (entry.path().is_file()) && (f_exten == "json") {
            //must be file cause could be directory
            //    println!("json data path: {}", entry.path().display());

            let data = fs::read_to_string(path).expect("Unable to read json data from path");

            // A JSON deserializer. You can use any Serde Deserializer here.
            let mut deserializer = serde_json::Deserializer::from_str(&data);

            // A compacted JSON serializer. You can use any Serde Serializer here.
            let mut serializer = ron::ser::Serializer::new(io::stdout() , Some(my_config)).unwrap();

            // Prints `{"a boolean":true,"an array":[3,2,1]}` to stdout.
            // This line works with any self-describing Deserializer and any Serializer.
            serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
        }
    }

    //let mut deserializer = serde_json::Deserializer::from_str(input);

   // println!("{:#?}", serde_data);
}

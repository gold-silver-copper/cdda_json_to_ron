use cdda_json_to_ron::*;

fn main() {
    let serde_data = SERDEdata::new("./assets/data/");

    println!("{:#?}", serde_data);
}

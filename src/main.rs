use serde::Deserialize;
use serde::Serialize;
use std::fs;
use toml;

#[derive(Deserialize, Serialize)]
struct Hex {
    number : u16,
    lines : String,
    name : String,
    pinyin : String,
    judgement : String,
    judgement_comm : String,
    image : String,
    image_comm : String,
    line_1 : String,
    line_1_comm : String,
    line_2 : String,
    line_2_comm : String,
    line_3 : String,
    line_3_comm : String,
    line_4 : String,
    line_4_comm : String,
    line_5 : String,
    line_5_comm : String,
    line_6 : String,
    line_6_comm : String
}

#[derive(Deserialize, Serialize)]
struct Changes {
    hexagram : Vec<Hex>
}

fn main() {
    let book_str = fs::read_to_string("./wilhelm_baynes.toml")
                     .expect("Could not open wilhelm_baynes.toml");
    let book : Changes = toml::from_str(&book_str).unwrap();
    println!("{}", book.hexagram[0].judgement_comm);

    // let t = Changes {
        // hexagram : vec![Hex{
                             // number : 1,
                             // lines : "000000".to_string(),
                             // name : "test".to_string(),
                             // pinyin : "pin".to_string(),
                             // judgement : "success".to_string(),
                             // judgement_comm : "j comm".to_string(),
                             // image : "good".to_string(),
                             // image_comm : "good comm".to_string(),
                             // line_1 : "line 1".to_string(),
                             // line_1_comm : "line 1 comm".to_string(),
                             // line_2 : "line 2".to_string(),
                             // line_2_comm : "line 2 comm".to_string(),
                             // line_3 : "line 3".to_string(),
                             // line_3_comm : "line 3 comm".to_string(),
                             // line_4 : "line 4".to_string(),
                             // line_4_comm : "line 4 comm".to_string(),
                             // line_5 : "line 5".to_string(),
                             // line_5_comm : "line 5 comm".to_string(),
                             // line_6 : "line 6".to_string(),
                             // line_6_comm : "line 6 comm".to_string()
                         // }, Hex {
                             // number : 2,
                             // lines : "111111".to_string(),
                             // name : "test".to_string(),
                             // pinyin : "pin".to_string(),
                             // judgement : "success".to_string(),
                             // judgement_comm : "j comm".to_string(),
                             // image : "good".to_string(),
                             // image_comm : "good comm".to_string(),
                             // line_1 : "line 1".to_string(),
                             // line_1_comm : "line 1 comm".to_string(),
                             // line_2 : "line 2".to_string(),
                             // line_2_comm : "line 2 comm".to_string(),
                             // line_3 : "line 3".to_string(),
                             // line_3_comm : "line 3 comm".to_string(),
                             // line_4 : "line 4".to_string(),
                             // line_4_comm : "line 4 comm".to_string(),
                             // line_5 : "line 5".to_string(),
                             // line_5_comm : "line 5 comm".to_string(),
                             // line_6 : "line 6".to_string(),
                             // line_6_comm : "line 6 comm".to_string()
                         // }
                        // ]
    // };
    // let ser = toml::to_string(&t).unwrap();
    // println!("{}", ser);
}

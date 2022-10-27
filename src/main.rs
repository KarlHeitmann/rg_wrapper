use crate::tipo::Tipo;

use std::fmt::{Display, Formatter, Result as FmtResult};
// use std::fmt::{Display, Formatter, Result};
// use std::fmt::{Display, Formatter};
use std::collections::HashMap;

mod io;
mod tipo;

pub struct Results {
    // pub data: Vec<String>,
    pub data: Vec<json::JsonValue>,
    // pub data_json: Vec<String>
    // pub data: Vec<&str>
    // pub data: HashMap<String, String>
}

/*
// impl contiene las implementaciones, las funciones/métodos que tendrá la estructura
impl Results {
    pub fn new(data_raw: Vec<&str>) -> Self { // Self con mayusculas es un alias para nombre struct
    // pub fn new(data_raw: &str) -> Self { // Self con mayusculas es un alias para nombre struct
    // pub fn new(data_raw: String) -> Self { // Self con mayusculas es un alias para nombre struct
        // data_raw.split(
        // let data: <Vec<&str>> = data_raw.split("\n").collect();
        // let data = data_raw.split("\n").collect::<Vec<&str>>();
        // let data = data_raw.split("\n").map(|&x| x.to_string()).collect::<Vec<String>>();
        let data = data_raw.split("\n").collect::<Vec<String>>();
        Self {
            // addr: addr //
            data // atajo para el de arriba
        }
    }

}
*/

impl Results {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Vec<String> = vec![];
        // let mut js: Vec<String> = vec![];
        let mut js: Vec<json::JsonValue> = vec![];

        for d in data_raw {
            v.push(d.to_string());
            // js.push(json::parse(d));
            // js.push(json::parse(d)).;
            let s = match  json::parse(d) {
            // let s = match str::from_utf8(&output.stdout) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            js.push(s);

            /*
            let parsed = json::parse(d);
            js.push(parsed);
            */

/*
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}"#
*/
        }
        Self {
            data: js
        }
    }
}



struct Match {
    r#type: Tipo,
}

impl Display for Results {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // let mut s = String();
        let mut string = String::from("");

        for d in &self.data {
            // string.push_str("hola");
            // string.push_str("{}", d);
            // let num: String = d["type"].try_into().expect("String value");
            // let num: String = d["type"].to_string();
            // let num: &str = &d["type"].to_string();
            let num = &d["type"].to_string();
            // let num: &str = d["type"];
            let m: Tipo = num.parse()?;
            // let m = num.parse()?;
            // let m: Result<Tipo, _> = num.parse();
            // let mm: Tipo = m?;
            // string.push_str(&num);
        }
        // write!(f, "{}", *self.data)
        // write!(f, "asd")

        write!(f, "asd {}", string)
        // string
        // write!(f, "{}", *self as u16) // how do we cast it? Cuidado con castear la referencia en vez
                                     // del valor. Dereference. No implementa Copy trait. Por eso
                                     // tenemos que hacer algo mas. Cuidado con diferencia entre
                                     // Copy y Clone. derive Copy arriba
    }
}

fn main() {
    println!("Hello, world!");

    let results = io::run_command();
    println!("{}", results);

    let r = Results::new(results.split("\n").collect::<Vec<&str>>());
    println!("r: {}", r);

}



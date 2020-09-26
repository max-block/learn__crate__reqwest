use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let res =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;

    println!("{:?}", res);

    Ok(())
}

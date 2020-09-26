use std::{collections::HashMap, error::Error};

use serde_json::Value;

fn t1() -> Result<(), Box<dyn Error>> {
    let res =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;

    println!("{:?}", res);
    Ok(())
}

fn t2() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://httpbin.org/get?q=1")?.json::<Value>()?;

    println!("{:?}", res.pointer("/args/q").unwrap());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    t1()?;
    t2()?;
    Ok(())
}

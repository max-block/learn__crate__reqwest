

use serde_json::Value;

fn main() {
    // let res = reqwest::blocking::get("https://crates.io/api/v1/crates/url");
    let res = reqwest::blocking::get("https://crates.io");
    dbg!(&res);
}
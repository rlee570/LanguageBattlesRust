#![feature(proc_macro_hygiene,decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;
use std::ops::Add;


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


#[get("/greet/<name>")]
fn greet(name: String) -> String {
    let greet = String::from("Hello ");
    greet.add(name.as_ref())
}

#[get("/pi?<decimals>")]
fn calculate_pi(decimals:i64) -> String{
    let mut divisor = 1.0;
    let mut result: f64 = 0.0;
    let precision:usize = decimals as usize;
    for i in 0..1000_000 {
        let sub_result = 4.0 / divisor;

        if i % 2 == 0 {
            result = result + sub_result;
        }
        else {
            result = result - sub_result;
        }

        divisor = divisor + 2.0;
    }
    format!("{:.*}",precision,result)
}

//#[post("/reverse",format="text/plain",data="<s>")]
//fn reverse(s: String) -> String{
//   s.chars().into_iter().rev().collect()
//}


fn main() {
    rocket::ignite().mount("/",routes![greet,calculate_pi]).launch();
}


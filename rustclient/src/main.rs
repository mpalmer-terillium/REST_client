extern crate hyper;

use hyper::*;
use std::io::Read;


fn main() {

    let client = Client::new();
    // this is wherever it is running...may need to find IP address
    let mut res = client.get("http://localhost:7101/RESTService/TestREST/weather/Chicago:IL:60115").send().unwrap();

    assert_eq!(res.status, hyper::Ok);

    let mut message = String::new(); 
    res.read_to_string(&mut message).unwrap();

    println!("{}", message);
}


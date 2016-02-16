extern crate iron;
extern crate rustc_serialize;
extern crate router;


use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Mutex, Arc};


#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}



fn main() {

    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, WORLD".to_string() }));
    let greeting_clone = greeting.clone();

    let mut router = Router::new();

    router.get("/", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap()));
    router.post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));

    
    fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
   
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request, greeting: &mut Greeting) -> IronResult<Response> {
       
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        *greeting = json::decode(&payload).unwrap();

        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}





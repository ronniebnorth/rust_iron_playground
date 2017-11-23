extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

const INDEX_HTML: &'static [u8] = include_bytes!("static/index.html");

fn main() {
  println!("Hello, world!");
  Iron::new(| _: &mut Request| {
    let content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, INDEX_HTML)))
  }).http("localhost:3000").unwrap();
}

/*
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
  Iron::new(|_: &mut Request| {
    Ok(Response::with((status::Ok, "Hello World!")))
  }).http("localhost:3000").unwrap();
}
*/
/*
extern crate rand;
use rand::{thread_rng, Rng};
fn main() {
  let mut rng = thread_rng();
  let mut y = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10
  ];
  rng.shuffle(&mut y);
  println!("{:?}", y);
}
*/
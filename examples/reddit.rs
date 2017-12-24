
#[macro_use]
extern crate wrest;

use wrest::rest::prelude::*;
use wrest::rest::methods::*;
use wrest::request::*;
use wrest::apipath::*;

fn main() {
    let mut reddit = REST::new("http://www.reddit.com");
    reddit.register(GET, APIPath::new("/r/rust/top/.json"), "rustsub_top");
    let output = reddit
                    .call("rustsub_top")
                    .params(args!("count" => "20"))
                    .run();
}

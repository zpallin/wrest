
#[macro_use]
extern crate wrest;

use wrest::prelude::*;

fn main() {
    let mut reddit = REST::new("http://www.reddit.com");
    reddit
        .register(
            GET, 
            APIPath::new("/r/rust/top/.json"), 
            "rustsub_top"
        );
    let mut output = reddit
        .call("rustsub_top")
        .params(args!("count" => "20"))
        .run()
        .unwrap();

    println!("{}", output.text().unwrap());
}

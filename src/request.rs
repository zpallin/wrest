
use std::io::Read;
use env_logger;
use reqwest;
use error_chain;
use std;

use std::collections::HashMap;
use rest::Action;

pub trait Call {
    fn call(&self, &str) -> Request;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    root : String,
    action : Action,
    path : String,
    data : String,
    params : String,
    headers : String,
}

impl Request {
    pub fn new(root : &str, action : Action) -> Self {
        Request { 
            root : root.to_string(),
            action : action.clone(),
            path : action.path.path,
            data : "".to_string(),
            params : "".to_string(),
            headers : "".to_string(),
        }
    }

    /// for parsing action parameters
    pub fn with(&self, args : HashMap<&str, &str>) -> Self {
        Request {
            root : self.root.clone(),
            action : self.action.clone(),
            path : self.action.path.parse(args),
            data : self.data.clone(),
            params : self.params.clone(),
            headers : self.headers.clone(),
        }
    }

    /// for passing url params
    pub fn params(&self, params : HashMap<&str, &str>) -> Self {
        Request {
            root : self.root.clone(),
            action : self.action.clone(),
            path : self.path.clone(),
            data : self.data.clone(),
            params : self.parse_params(params),
            headers : self.headers.clone(),
        }
    }

    /// for passing headers into the request
    pub fn headers(&self, headers : HashMap<&str, &str>) -> Self {
        Request {
            root : self.root.clone(),
            action : self.action.clone(),
            path : self.path.clone(),
            data : self.data.clone(),
            params : self.params.clone(),
            headers : self.parse_headers(headers),
        }
    }

    /// for passing data in POST, PUT
    pub fn data(&self, data : &str) -> Self {
        Request {
            root : self.root.clone(),
            action : self.action.clone(),
            path : self.path.clone(),
            data : data.to_string(),
            params : self.params.clone(),
            headers : self.headers.clone(),
        }
    }

    pub fn parse_params(&self, params : HashMap<&str, &str>) -> String {
        let mut out : String = "".to_string();
        for (key, val) in params {
            out = format!("{},{}={}", out, key, val);
        }

        // add the "?" if the string is not empty
        if out != "" {
            format!("?{}", out)
        } else {
            out
        }
    }

    pub fn parse_headers(&self, headers : HashMap<&str, &str>) -> String {
        let mut out : String = "".to_string();
        for (key, val) in headers {
            out = format!("{},{}: {}", out, key, val);
        }
        out
    }

    /// carries out the request action as defined
    pub fn run(&self) -> Result<String, ()> {
        env_logger::init().expect("Failed to initialize logger");
       
        let url = format!("{}{}{}", self.root, self.path, self.params);
        let mut res = reqwest::get(&url).unwrap();
        
        println!("Status: {}", res.status());
        println!("Headers:\n{}", res.headers());

        let _ = std::io::copy(&mut res, &mut std::io::stdout()).unwrap();
        println!("\n\nDone.");

/*
        let url = url.parse::<hyper::Uri>().unwrap();
        if url.scheme() != Some("http") {
            println!("This example only works with 'http' URLs.");
            return Err(());
        }

        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .no_proto()
            .build(&handle);

        let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
            }).map(|_| {
                println!("\n\nDone.");
            });

        core.run(work).unwrap();
*/
        Ok("hello".to_string())
    }
}

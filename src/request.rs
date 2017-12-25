
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
        let mut out = params
                        .iter()
                        .map(|(k,v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join(",");
        
        // add the "?" if the string is not empty
        if out != "" {
            format!("?{}", out)
        } else {
            out.to_string()
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
    pub fn run(&self) -> Result<reqwest::Response, reqwest::Error> {
        env_logger::init().expect("Failed to initialize logger");
       
        let url = format!("{}{}{}", self.root, self.path, self.params);
        let mut res = reqwest::get(&url)?;
        
//        println!("Status: {}", res.status());
//        println!("Headers:\n{}", res.headers());
//        println!("Output: \n{}", res.text().unwrap());

        //let _ = std::io::copy(&mut res, &mut std::io::stdout()).unwrap();
        Ok(res)
    }
}

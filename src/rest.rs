
use apipath::*;
use request::*;

pub mod prelude {
    pub use rest::{REST, APIMethod, Action};
    pub use request::Request;
}

pub mod methods {
    pub use rest::APIMethod::*;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum APIMethod {
    POST,
    PUT,
    GET,
}

/// an Action is used to derive functionality for an api call
/// and help define how it should be used. It declares a shortcut
/// name that can be used to call it, as well as a delimited http path
/// that can have variable inserts via the APIPath struct. It also
/// comes with an APIMethod for explicit request methods, GET, POST, PUT, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub name : String,
    pub path : APIPath,
    pub method : APIMethod,
}

//impl<'b, 'c> Clone for Action<'b, 'c> {
//    fn clone(&self) -> Self {
//        Action {
//            name : self.name,
//            path : self.path.clone(),
//            method : self.method,
//        }
//    }
//}

/// REST struct is the center of all client interactions with the api.
/// A REST struct is used to generate registered actions with
///
#[derive(Serialize, Deserialize, Debug)]
pub struct REST {
    pub root : String,
    pub actions : Vec<Action>,
}

impl Call for REST {
    fn call(&self, name: &str) -> Request {
        Request::new(
            &self.root,
            self.actions.iter()
                        .filter(|x| x.name == name.to_string())
                        .next()
                        .unwrap()
                        .clone()
        )
    }
}

impl REST {
    pub fn new(root : &str) -> Self {
        let actions : Vec<Action> = Vec::new();
        REST { 
            root : root.to_string(),
            actions : actions
        }
    }
}

/// REST functionality handles registry of actions
///
impl REST {
    pub fn register(&mut self, method : APIMethod, path : APIPath, name : &str) {
        self.actions.push(Action {
                name : name.to_string(),
                path : path,
                method : method,
            });
    }
}

#[cfg(test)]
mod tests {
    use rest::prelude::*;
    use rest::methods::*;
    use apipath::*;
    use request::*;
    
    #[cfg(test)]
    use mockito;

    #[test]
    fn it_runs() {
        let _m = mockito::mock("GET", "/user/zpallin")
          .with_status(201)
          .with_header("content-type", "text/plain")
          .with_header("x-api-key", "1234")
          .with_body("<html>test</html>")
          .create();

        let mut fakesite = REST::new(mockito::SERVER_URL);
        fakesite
            .register(
                POST, 
                APIPath::new("/user/{:id}"), 
                "getuser"
            );

        let mut res : Result<Response, Error> = fakesite
            .call("getuser")
            .with(args!(":id" => "zpallin"))
            .headers(args!("User-Agent" => "test-user"))
            .run();
       
        let body = res
            .unwrap()
            .text()
            .unwrap();

        assert_eq!("<html>test</html>", format!("{}", body));
    }

    #[test]
    fn it_can_post() {
        
    }
}

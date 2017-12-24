
use std::collections::HashMap;

/// APIPath struct that provides a simple interface for registering api syntax
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIPath {
    pub path : String,
}

impl APIPath {
    pub fn new(path : &str) -> Self {
        APIPath {
            path : path.to_string()
        }
    }
    pub fn parse(&self, args : HashMap<&str, &str>) -> String {
        let mut out : String = self.path.to_string();

        for (key, value) in &args {
            let keyfmt = &format!("{{{}}}", key);
            out = out.replace(keyfmt, value);
        }
        out.to_owned()
    }
}

/// args!() macro just returns a HashMap<&str, &str>, but this is to
/// provide a shipped macro that makes it semantically easier to register
/// api actions.
#[macro_export]
macro_rules! args {
    ( $($key:expr => $val:expr),* ) => {{
        use std::collections::HashMap;
        let mut map : HashMap<&str, &str> = HashMap::new();
        $(
            map.entry($key).or_insert($val);
        )*
        map
    }};
}

#[cfg(test)]
mod tests {
    use apipath::*;

    #[test]
    fn it_builds() {
        let req = APIPath::new("/r/rust");
    }

    #[test]
    fn it_takes_an_argument_template() {
        //using reddit as an example, taking subreddit as an argument for a request
        let path = APIPath::new("/r/{:id}");
        let out : String = path.parse(args!(":id" => "rust"));
        assert_eq!("/r/rust", &out);
    }

    #[test]
    fn it_takes_multi_arg_template() {
        // same as above, but now multiple
        let path = APIPath::new("/user/{:userid}/posts/{:postid}");
        let out : String = path.parse(args!(
                                ":userid" => "zpallin",
                                ":postid" => "9001"
                                ));
        assert_eq!("/user/zpallin/posts/9001", &out);
    }
}

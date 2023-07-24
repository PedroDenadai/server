use super::method::Method;

        pub struct Request{
            path: String,
            query_string: Option<String>,
            method: Method, // use super because of the scope of the struct 
        }
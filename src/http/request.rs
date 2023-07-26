use std::str;
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};



pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method, // use super because of the scope of the struct 
}


impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from<'a>(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error>{
        //match str::from_utf8(buf){
            //Ok(request) => {}
            //Err(_) => return Err(ParseError::InvalidEncoding),}

        //match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
          //  Ok(request) => {}
           // Err(e) => return Err(e),}

        let request = str::from_utf8(buf)?;  

        //match get_word(request) {
        //    Some((method, request)) => {},
        //    None => return Err(ParseError::InvalidRequest),
        //}
        let (method, request) = get_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_word(request).ok_or(ParseError::InvalidRequest)?; 
        let (protocol, _) = get_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        //match path.find('?') {
          //  Some(i) => {
            //    query_string = Some(&path[i + 1..]);
              //  path = &path[..i];
            //}

//            None => {},
//      }

        //let q = path.find('?');
        //if q.is_some() {
          //  let i = q.unwrap();
            //query_string = Some(&path[i + 1..]);
            //path = &path[..i];
        //}

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];

        }

        Ok(Self{
            path,
            query_string,
            method,
        })



 
    }
}


//recebe uma str e retorna um tuple que tem 2 valores, 1 que é a str de interesse (ate um espaco) e o resto da str
fn get_word<'a>(request: &'a str) -> Option<(&'a str, &'a str)>{ // the lifetime of both str that returns has to have the some lifetime
    //let mut iter = request.chars();
    //loop {
    //    let item = iter.next();
      //  match item {
  //          Some(c) => {},
//            None => break,
      //  }
    //}

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None

}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}


impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//impl Error for ParseError{

//}
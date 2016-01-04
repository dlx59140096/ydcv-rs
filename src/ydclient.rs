//! ydclient is client wrapper for Client

use std::error::Error;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use ::Client;
use url::Url;

use super::ydresponse::YdResponse;

/// API name from ydcv
const API: &'static str = "YouDaoCV";

/// API key from ydcv
const API_KEY: &'static str = "659600698";

/// Wrapper trait on `hypper::Client`
pub trait YdClient{
    /// lookup a word on YD and returns a `YdPreponse`
    ///
    /// # Examples
    ///
    /// lookup "hello" and compare the result:
    ///
    /// ```
    /// assert_eq!("YdResponse('hello')",
    ///        format!("{}", Client::new().lookup_word("hello").unwrap()));
    /// ```
    fn lookup_word(&mut self, word: &str, raw: bool) -> Result<YdResponse, Box<Error>>;
    fn decode_result(&mut self, result: &str) -> Result<YdResponse, Box<Error>>;
}

/// try and rethrow the possible error in `Box<Error>`
macro_rules! try_box {
    ($expr:expr) => (match $expr {
        Ok(val) => Ok(val),
        Err(err) => Err(Box::new(err))
    })
}

/// Implement wrapper client trait on `hypper::Client`
impl YdClient for Client {

    fn decode_result(&mut self, result: &str) -> Result<YdResponse, Box<Error>> {
        debug!("Recieved JSON {}", Json::from_str(&result).unwrap().pretty());
        try_box!(json::decode::<YdResponse>(&result))
    }

    #[cfg(feature="hyper")]
    /// lookup a word on YD and returns a `YdPreponse`
    fn lookup_word(&mut self, word: &str, raw: bool) -> Result<YdResponse, Box<Error>> {
        use std::io::Read;

        let mut url = try!(Url::parse("http://fanyi.youdao.com/openapi.do"));
        url.set_query_from_pairs(vec!(("keyfrom", API),
            ("key", API_KEY), ("type", "data"), ("doctype", "json"),
            ("version", "1.1"), ("q", word)).into_iter());

        let mut body = String::new();

        try!(
            try!(self.get(&url.serialize()).send())
            .read_to_string(&mut body));

        let raw_result = YdResponse::new_raw(body);
        if raw {
            Ok(raw_result)
        }else{
            self.decode_result(&raw_result.raw_result())
        }
    }

    #[cfg(feature="curl")]
    /// lookup a word on YD and returns a `YdPreponse`
    fn lookup_word(&mut self, word: &str, raw: bool) -> Result<YdResponse, Box<Error>> {
        let mut url = try!(Url::parse("http://fanyi.youdao.com/openapi.do"));
        url.set_query_from_pairs(vec!(("keyfrom", API),
            ("key", API_KEY), ("type", "data"), ("doctype", "json"),
            ("version", "1.1"), ("q", word)).into_iter());

        let resp = self.handle
                .get(url.serialize())
                .exec().unwrap();
        let body = String::from_utf8_lossy(resp.get_body()).clone();

        let raw_result = YdResponse::new_raw(body.into_owned());
        if raw {
            Ok(raw_result)
        }else{
            self.decode_result(&raw_result.raw_result())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Client;

    #[test]
    fn test_lookup_word_0(){
        assert_eq!("YdResponse('hello')",
            format!("{}", Client::new().lookup_word("hello", false).unwrap()));
    }

    #[test]
    fn test_lookup_word_1(){
        assert_eq!("YdResponse('world')",
            format!("{}", Client::new().lookup_word("world", false).unwrap()));
    }

    #[test]
    fn test_lookup_word_2(){
        assert_eq!("YdResponse('<+*>?_')",
            format!("{}", Client::new().lookup_word("<+*>?_", false).unwrap()));
    }
}

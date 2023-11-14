#[derive(Debug, PartialEq)]
pub enum Method{
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method{
    fn from(value: &str) -> Method {
        match value.to_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version{
    fn from(value: &str) -> Version {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource{
    Path(String),
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        let m: Method = "Get".into();
        assert_eq!(m, Method::Get)
    }
}
/*
- Configuration for the program 

*/
use serde::Deserialize; 
use std::fmt;

#[derive(Deserialize)]
pub struct Config {
   pub key: Key,
   pub domain: Domain,
   pub dnsrecord: Dnsrecord,
   pub endpoint: Endpoint,

}

#[derive(Deserialize)]
pub struct Key {
    apikey: String

}
#[derive(Deserialize)]
pub struct Domain {
    domain: String

}
#[derive(Deserialize)]
pub struct Dnsrecord {
    a_name: String

}
#[derive(Deserialize)]
pub struct Endpoint {
    api: String

}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.apikey)
    }
    
}
impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.domain)
    }
    
}
impl fmt::Display for Dnsrecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.a_name)
    }
    
}
impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.api)
    }
    
}
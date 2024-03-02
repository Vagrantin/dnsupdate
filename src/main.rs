/*
- DISCLAMER : BoilerPlate with BING AI tool ( ChatGPT )
----- steps -----
- Get public IP
- publish new IP to Gandi API


*/
//Local stuff
mod models;

use std::{env, fs};
use toml;
use models::Config;
//public crate
use reqwest::{Error, header::{HeaderMap, HeaderValue}}; // Import the reqwest crate
use serde::Serialize; 
use serde_json::json;
use std::process::exit;

fn main() {
let args: Vec<String> = env::args().collect();
//dbg!(args);
if &args.len() < &2 {
  println!("you are missing the config file");
  exit(0);
}
let filepath = &args[1];
//println!("Path to the file: {}", filepath);
#[tokio::main] // Use the tokio runtime
async fn dnsupdate(filepath:&String) -> Result<(), Error> {
let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
let config: Config = toml::from_str(&contents).unwrap();
let key = config.key.to_string();
let domain = config.domain.to_string();
let dnsrecord = config.dnsrecord.to_string();
let endpoint = config.endpoint.to_string();
let url = format!("{endpoint}{domain}/records/{dnsrecord}/A");

//println!("This should be the APIKey: {}", config.key);

let response = reqwest::get("https://api.ipify.org").await?;  // Make a GET request
let ip = response.text().await?;  // Get the response body as text
//println!("My public IP address is: {}", ip);  // Print the IP address
let ipstringslice = json!({"rrset_values": [ip]});
//println!("Body in JSON: {}", ipstringslice.to_string());
let mut headers = HeaderMap::new();
headers.insert("CONTENT_TYPE", HeaderValue::from_str("application/json").unwrap()); // Set a custom header
headers.insert("X-Api-Key", HeaderValue::from_str(&key).unwrap()); // Set a custom header
let client = reqwest::Client::new(); // Create a new client
let res = client
.put(&url) // Create a PUT request to update user 1
.headers(headers) // Set a custom header
.body(ipstringslice.to_string())
.send() // Send the request asynchronously
.await?; // Wait for the response

//println!("Status: {}", res.status()); // Print the status code
//println!("Headers:\n{:#?}", res.headers()); // Print the headers
let body = res.text().await?; // Get the response body as text
//println!("Body:\n{}", body); // Print the body

Ok(())
}
let _ = dnsupdate(filepath);
}

// A custom type that can be serialized into JSON
#[derive(Serialize,Debug)]
struct Payload<'a> {
rrset_values: &'a str,
rrset_ttl: &'a  str,
}

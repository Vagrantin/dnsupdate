/*
- DISCLAMER : BoilerPlate with BING AI tool ( ChatGPT )
----- steps -----
- Get public IP
- publish new IP to Gandi API


*/
use reqwest::{Error, header::{HeaderMap, HeaderValue}}; // Import the reqwest crate
use serde::Serialize; //not sure what I’m doing here 
use serde_json::json;

#[tokio::main] // Use the tokio runtime
async fn main() -> Result<(), Error> {
let response = reqwest::get("https://api.ipify.org").await?;  // Make a GET request
let ip = response.text().await?;  // Get the response body as text
println!("My public IP address is: {}", ip);  // Print the IP address
let ipstringslice = json!({"rrset_values": [ip]});
println!("Body in JSON: {}", ipstringslice.to_string());
let mut headers = HeaderMap::new();
headers.insert("CONTENT_TYPE", HeaderValue::from_str("application/json").unwrap()); // Set a custom header
headers.insert("X-Api-Key", HeaderValue::from_str("vSk6HubiW2KYyZhcfgC8LnbV").unwrap()); // Set a custom header
let client = reqwest::Client::new(); // Create a new client
let res = client
.put("https://dns.api.gandi.net/api/v5/domains/ducorps.fr/records/dalet/A") // Create a PUT request to update user 1
.headers(headers) // Set a custom header
.body(ipstringslice.to_string())
.send() // Send the request asynchronously
.await?; // Wait for the response

println!("Status: {}", res.status()); // Print the status code
println!("Headers:\n{:#?}", res.headers()); // Print the headers
let body = res.text().await?; // Get the response body as text
println!("Body:\n{}", body); // Print the body

Ok(())
}

// A custom type that can be serialized into JSON
#[derive(Serialize)]
#[derive(Debug)]
struct Payload<'a> {
rrset_values: &'a str,
rrset_ttl: &'a  str,
}
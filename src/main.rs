extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;


#[derive(Debug, Deserialize)]
struct DnsResponseAnswer {
    name: String,
    #[serde(rename = "type")]
    record_type: i32,
    #[serde(rename = "TTL")]
    ttl: i32,
    data: String,
}

#[derive(Debug, Deserialize)]
struct DnsResponse {
    #[serde(rename = "Status")]
    status: i32,
    #[serde(rename = "TC")]
    tc: bool,
    #[serde(rename = "RD")]
    rd: bool,
    #[serde(rename = "RA")]
    ra: bool,
    #[serde(rename = "AD")]
    ad: bool,
    #[serde(rename = "CD")]
    cd: bool,
    #[serde(default)]
    #[serde(rename = "Answer")]
    answer: Vec<DnsResponseAnswer>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let domain = args[1].as_str();

        let record_types = [["A", "has address"], ["AAAA", "has IPv6 address"], ["MX", "mail is handled by"]];

        for record_type in record_types.iter() {
            let data = match dns_query(domain, record_type[0]) {
                Ok(data) => data,
                Err(error) => {
                    println!("{}", error);
                    DnsResponse {
                        status: 0,
                        tc: false,
                        rd: false,
                        ra: false,
                        ad: false,
                        cd: false,
                        answer: vec![],
                    }
                }
            };

            for answer in data.answer {
                println!("{} {} {}", answer.name, record_type[1], answer.data);
            }
        }
    }
}

fn dns_query(domain: &str, record_type: &str) -> Result<DnsResponse, &'static str> {
    let client = reqwest::Client::new();
    let url = ["https://cloudflare-dns.com/dns-query?name=", domain, "&type=", record_type].join("").to_string();

    let mut response = client.get(&url)
        .header("accept", "application/dns-json")
        .send()
        .expect("Failed to send request");

    if let Ok(data) = response.json::<DnsResponse>() {
        return Ok(data);
    }
    return Err("Error");
}

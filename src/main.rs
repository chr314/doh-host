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
    #[serde(rename = "Answer")]
    answer: Vec<DnsResponseAnswer>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let domain = args[1].as_str();

        let record_a = dns_query(domain, "A");
        for answer in record_a.answer {
            println!("{} has address {}", answer.name, answer.data);
        }

        let record_aaaa = dns_query(domain, "AAAA");
        for answer in record_aaaa.answer {
            println!("{} has IPv6 address {}", answer.name, answer.data);
        }

        let record_mx = dns_query(domain, "MX");
        for answer in record_mx.answer {
            println!("{} mail is handled by {}", answer.name, answer.data);
        }
    }
}

fn dns_query(domain: &str, record_type: &str) -> DnsResponse {
    let client = reqwest::Client::new();
    let url = ["https://cloudflare-dns.com/dns-query?name=", domain, "&type=", record_type].join("").to_string();

    let mut response = client.get(&url)
        .header("accept", "application/dns-json")
        .send()
        .expect("Failed to send request");

    if let Ok(data) = response.json::<DnsResponse>() {
        return data;
    }
    return DnsResponse { status: 0, tc: false, rd: false, ra: false, ad: false, cd: false, answer: vec![] };
}
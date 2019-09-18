#[macro_use]
extern crate serde_derive;

use std::env;

mod dns_query;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let domain = args[1].as_str();

        let record_types = [["A", "has address"], ["AAAA", "has IPv6 address"], ["MX", "mail is handled by"]];

        for record_type in record_types.iter() {
            let data = match dns_query::query(domain, record_type[0]) {
                Ok(data) => data,
                Err(error) => {
                    println!("{}", error);
                    dns_query::Response {
                        status: 0,
                        tc: false,
                        rd: false,
                        ra: false,
                        ad: false,
                        cd: false,
                        question: vec![],
                        authority: vec![],
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

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "TC")]
    pub tc: bool,
    #[serde(rename = "RD")]
    pub rd: bool,
    #[serde(rename = "RA")]
    pub ra: bool,
    #[serde(rename = "AD")]
    pub ad: bool,
    #[serde(rename = "CD")]
    pub cd: bool,
    #[serde(rename = "Question")]
    pub question: Vec<Question>,
    #[serde(default)]
    #[serde(rename = "Authority")]
    pub authority: Vec<Authority>,
    #[serde(default)]
    #[serde(rename = "Answer")]
    pub answer: Vec<Answer>,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    name: String,
    #[serde(rename = "type")]
    record_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: i32,
    #[serde(rename = "TTL")]
    pub ttl: i32,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct Authority {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: i32,
    #[serde(rename = "TTL")]
    pub ttl: i32,
    pub data: String,
}

pub fn query(domain: &str, record_type: &str) -> Result<Response, &'static str> {
    let client = reqwest::Client::new();
    let url = ["https://cloudflare-dns.com/dns-query?name=", domain, "&type=", record_type].join("").to_string();

    let mut response = client.get(&url)
        .header("accept", "application/dns-json")
        .send()
        .expect("Failed to send request");

    if let Ok(data) = response.json::<Response>() {
        return Ok(data);
    }
    return Err("Error");
}

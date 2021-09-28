
use reqwest::{header, Client, Url};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpcResponse {
        success: bool,
        #[serde(flatten)]
        payload: UpcPayload
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum UpcPayload {
        Data(UpcData),
        Err(UpcError)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpcData {
        barcode: String,
        title: String,
        alias: String,
        description: String,
        brand:  String,
        manufacturer: String,
        mpn: String,
        ASIN: String,
        category: String,
}

fn default_time() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpcError {
        error: UpcErrorDetails,
        //#[serde(default="default_time")]
        //time: u64
}

fn default_error_code() -> i32 {
        400
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpcErrorDetails {
        #[serde(default="default_error_code")]
        code: i32,
        message: String,
        //#[serde(default)]
        //endpoint: String,
        //#[serde(default)]
        //query: String,
}

pub struct UpcApi {
        client: Client,
        token: String
}

impl UpcApi {
        pub fn new(token: String) -> UpcApi {
                UpcApi {
                        client: Client::new(),
                        token
                }
        }

        pub async fn lookup(&self, upc: &str) -> Result<UpcResponse, Box<dyn std::error::Error>> {
                let url = Url::parse("https://api.upcdatabase.org/product/")?.join(upc)?;
                Ok(self.client.get(url).basic_auth("", Some(&self.token)).send().await?.json::<UpcResponse>().await?)
        }
}

#[cfg(test)]
mod tests {
        use super::*;
        use dotenv::dotenv;

        #[test]
        fn deserialize_400_test() {
                let response = r#"
                {
                        "success": false,
                        "error": {
                                "code": 403,
                                "message": "Bad Request. The code you are trying to enter does not contain all digits."
                        }
                }"#;

                let s: UpcResponse = serde_json::from_str(response).unwrap();
                println!("{:?}", s);
                assert!(!s.success);
                match s.payload {
                        UpcPayload::Data(_) => assert!(false),
                        UpcPayload::Err(details) => assert_eq!(details.error.code, 403),
                }
        }

        #[test]
        fn serialize_400_test() {
                let x = UpcResponse {
                        success: false,
                        payload: UpcPayload::Err(UpcError {
                                error: UpcErrorDetails {
                                        code: 403,
                                        message: "Bad Request. The code you are trying to enter does not contain all digits".to_owned()
                                },
                        })
                };
                println!("{:?}", serde_json::to_string(&x).unwrap());
        }

        #[actix_rt::test]
        async fn lookup_test() {
                dotenv().ok();

                let api = UpcApi::new(std::env::var("UPC_TOKEN").unwrap());
                let thing = api.lookup("0021908115399").await.unwrap();
                println!("{:?}", thing);
                assert!(thing.success);
                match thing.payload {
                        UpcPayload::Data(data) => assert_eq!(data.title, "Larabar Apple Pie Protein Bar - 9.6oz/6ct"),
                        UpcPayload::Err(_) => assert!(false),
                }
        }
}
use reliakit_secret::{ExposeSecret, Secret, SecretString};

fn main() {
    let api_key = Secret::new("rk_live_example");
    let password = SecretString::from_string("correct horse battery staple");

    println!("api key: {api_key}");
    println!("password: {password:?}");
    println!("api key length: {}", api_key.expose_secret().len());
}

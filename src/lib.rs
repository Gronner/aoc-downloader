use std::fs;
use config::Config;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use reqwest::Url;

fn read_config() -> Config {
    let mut cookie_config = Config::default();
    cookie_config.merge(config::File::with_name("aoc"))
        .expect("Could not find session settings file: aoc.toml");
    cookie_config
}

fn build_client(config: Config) -> Result<Client, reqwest::Error> {
    let cookie = format!(
        "session={}",
        config
            .get_str("session")
            .expect("Could not read config value \"session\"")
    );
    
    let jar = std::sync::Arc::new(Jar::default());
    let url = "https://adventofcode.com".parse::<Url>().unwrap();

    jar.add_cookie_str(&cookie, &url);

    Client::builder()
        .cookie_provider(jar)
        .build()
}

pub fn download_day(day: u32, path: &str) -> Result<(), reqwest::Error> {
    if std::path::Path::new(&format!("{}/input{}.txt", path, day)).exists() {
        return Ok(());
    }

    let config = read_config();
    let year = config.get_int("year").expect("Could not read config value \"year\"");
    let client = build_client(config)?;
    let response = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .send()?
        .text()?;
    match std::fs::create_dir(path) {
        Ok(()) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("{:?}", e),
        },
    };
    fs::write(format!("{}/input{}.txt", path, day), response)
        .expect(&format!("Unable to write to file: {}", path));
    Ok(())
}

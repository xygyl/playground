use inquire::Text;
use std::str::from_utf8;

fn cleaner(url: String) -> String {
    url.as_bytes()
        .split(|c| c == &b'?')
        .next()
        .and_then(|c| from_utf8(c).ok())
        .unwrap_or(&url)
        .to_string()
}

pub fn link_cleaner() -> Option<()> {
    let url: String = Text::new("Enter url:")
        .prompt()
        .ok()
        .unwrap_or("Unable to clean link".to_string());
    println!("{}", cleaner(url));
    Some(())
}

use reqwest::header::USER_AGENT;
use crate::util;

pub struct CurrentlyPlaying {
    pub number: u16,
    pub artist: String,
    pub title: String,
    pub image: String,
}

pub fn fetch_currently_playing(client: &reqwest::blocking::Client) -> CurrentlyPlaying {
    let json = make_api_request(client).expect("Failed to fetch currently playing song");

    let currently_playing = &json["data"][0];
    let artist = util::get_str_or_default(&currently_playing, "artist", "Unknown Artist");
    let title = util::get_str_or_default(&currently_playing, "title", "Unknown Title");

    let (number, title) = util::parse_title(&*title);

    let image: String = util::get_str_or_default(&currently_playing, "image_url_200x200", "https://www.nporadio2.nl/images/unknown_programme.webp?h=200&w=200")
        .split('?')
        .next()
        .unwrap_or("")
        .to_string();

    CurrentlyPlaying {
        number,
        artist,
        title,
        image,
    }
}

fn make_api_request(client: &reqwest::blocking::Client) -> Result<serde_json::Value, reqwest::Error> {
    client.get("https://www.nporadio2.nl/api/tracks")
        .header(USER_AGENT, "nowaha-t2000-lfm/1.0")
        .send()?
        .json()
}
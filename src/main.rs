mod api;
mod util;
mod discord;

use rustfm_scrobble::{Scrobble, Scrobbler};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{env, io, thread, time};
use time::{Duration, SystemTime, UNIX_EPOCH};
use discord_game_sdk::{Activity, Discord};
use crate::api::CurrentlyPlaying;

fn login(scrobbler: &mut Scrobbler) {
    let mut success = false;
    let mut username = env::var("LASTFM_USERNAME").unwrap_or_default();
    let mut password = env::var("LASTFM_PASSWORD").unwrap_or_default();

    while !success {
        if username.is_empty() {
            println!("Enter your Last.fm username:");
            io::stdin().read_line(&mut username).expect("Failed to read line");
            username = username.trim().to_string();
        }

        if password.is_empty() {
            println!("Enter your Last.fm password: ");
            io::stdin().read_line(&mut password).expect("Failed to read line");
            password = password.trim().to_string();
        }

        let response = scrobbler.authenticate_with_password(&*username, &*password);
        match response {
            Ok(_) => success = true,
            Err(e) => {
                util::clear();
                println!("Failed to log in: {}", e);
                username.clear();
                password.clear();
            }
        }
    }

    util::clear();
    println!("Logged in successfully!");
}

fn main() {
    util::clear();
    dotenv::dotenv().ok();

    let api_key = env::var("LASTFM_API_KEY").expect("Missing LASTFM_API_KEY environment variable");
    let api_secret = env::var("LASTFM_API_SECRET").expect("Missing LASTFM_API_SECRET environment variable");

    let mut scrobbler = Scrobbler::new(&*api_key, &*api_secret);
    login(&mut scrobbler);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let client = reqwest::blocking::Client::new();
    let song_check_thread = thread::spawn(move || -> () {
        let mut discord_client = Discord::new(1056352600045387796).unwrap();
        *discord_client.event_handler_mut() = Some(discord::MyEventHandler);

        let mut started = 0;
        let mut last_track = String::new();
        let mut scrobbled = false;

        while running_clone.load(Ordering::SeqCst) {
            let CurrentlyPlaying { number, artist, title, image } = api::fetch_currently_playing(&client);
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            let track = Scrobble::new(&artist, &title, "");
            if last_track != title {
                last_track = title.clone();
                started = now;
                scrobbled = false;

                if let Err(err) = scrobbler.now_playing(&track) {
                    println!("Failed to scrobble: {}", err)
                } else {
                    println!("Now playing #{}: {} - {}", number, artist, title)
                }

                discord_client.update_activity(&Activity::empty()
                    .with_details(&*format!("#{}: {} - {}", number, artist, title))
                    .with_large_image_tooltip(&*format!("Track #{}: {} - {}", number, artist, title))
                    .with_large_image_key(&*image)
                    .with_start_time(now as i64), |_discord, res| {
                    if let Err(err) = res {
                        eprintln!("Failed to update activity: {}", err);
                    }
                });
            }

            if !scrobbled && now - started > 30 {
                scrobbled = true;
                if let Err(err) = scrobbler.scrobble(&track) {
                    println!("Failed to scrobble: {}", err)
                } else {
                    println!("Scrobbled #{}: {} - {}", number, artist, title)
                }
            }

            for _ in 0..10 {
                discord_client.run_callbacks().unwrap();
                thread::sleep(Duration::from_secs(1));
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
        }
    });

    let input_thread = thread::spawn(move || while running.load(Ordering::SeqCst) {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" {
                running.store(false, Ordering::SeqCst);
                println!("Exiting...");
            }
        }
    );

    song_check_thread.join().unwrap();
    input_thread.join().unwrap();
}

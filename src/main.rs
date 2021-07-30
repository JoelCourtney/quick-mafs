use discord::model::{Event, ReactionEmoji};
use discord::Discord;
use urlencoding::encode;
use std::fs::File;
use std::path::Path;
use downloader::{Downloader, Download};

static REACTIONS: [&'static str; 9] = [
    "\u{1f1f6}",
    "\u{1f1fa}",
    "\u{1f1ee}",
    "\u{1f1e8}",
    "\u{1f1f0}",
    "\u{1f1f2}",
    "\u{1f1e6}",
    "\u{1f1eb}",
    "\u{1f1f8}"
];

static mut EQUATION_INDEX: usize = 0;

fn main() {

    let file_contents= std::fs::read_to_string("equations.txt")
        .expect("Something went wrong reading the file");
    let equations: Vec<_> = file_contents.lines().collect();

    unsafe {
        EQUATION_INDEX = rand::random::<usize>() % equations.len();
    }

    let mut name = load_equation(&equations);

    let token = std::fs::read_to_string("token.txt").expect("could not read token");
    let discord = Discord::from_bot_token(&*token)
        .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Doing quick mafs");
    loop {

        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                let lower = message.content.to_lowercase();
                if lower.contains("quick mafs") || lower.contains("quick maffs") {
                    println!("summoned by {}", message.author.name);
                    let response = discord.send_file(
                        message.channel_id,
                        &format!("two plus two is four, minus one that's {}", name),
                        File::open("equation.png").unwrap(),
                        "equation.png"
                    ).unwrap();
                    for reaction in REACTIONS {
                        discord.add_reaction(
                            message.channel_id,
                            response.id,
                            ReactionEmoji::Unicode(reaction.to_string())
                        ).unwrap();
                    }
                    name = load_equation(&equations);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}

fn load_equation<'a>(equations: &Vec<&'a str>) -> &'a str {

    if Path::new("equation.png").exists() {
        std::fs::remove_file("equation.png").unwrap();
    }

    let (name, equation) = unsafe { equations[EQUATION_INDEX].split_once(" | ").unwrap() };
    let url = format!("https://latex.codecogs.com/png.latex?\\dpi{{150}}&space;\\fn_cm&space;\\LARGE&space;{}", encode(equation));

    let download = Download::new(&*url).file_name("equation.png".as_ref());
    Downloader::builder().build().unwrap().download(&[download]).expect("download failed");

    unsafe {
        EQUATION_INDEX += 1;
        if EQUATION_INDEX == equations.len() {
            EQUATION_INDEX = 0;
        }
    }

    name
}
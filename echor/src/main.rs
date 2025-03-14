use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Finlay Smyth <finadamsmyth@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            Arg::new("omit-newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("{:#?}", matches);

    let text: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();

    let omit_new_line: bool = matches.get_flag("omit-newline");

    let ending = if omit_new_line { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}

use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0a0")
        .author("francoposa <franco@francoposa.io>")
        .about("rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .help("omit trailing newline character")
                .takes_value(false),
        )
        .get_matches();

    // we can unwrap on text because clap guarantees we will have values
    // for the text argument if we reach this point in the program
    let text = matches
        .get_many("text")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();
    let omit_newline = matches.is_present("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending)
}

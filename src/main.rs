use clap::{App, Arg};

fn main() {
    let _matches = App::new("whey")
        .version("0.1.0")
        .author("Marcel Ferreira <marcel.ferreira@unesp.br>")
        .about("whey: protein analysis")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();

    println!("{:#?}", _matches)
}

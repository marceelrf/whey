use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "whey", author = "Marcel Ferreira", version = "0.1.0")]
#[command(about = "whey: Protein sequence analysis CLI", long_about = None,
    before_help = concat!(r#"            _                    "#, "\n",
                          r#"__      __ | |__     ___   _   _ "#, "\n",
                          r#"\ \ /\ / / | '_ \   / _ \ | | | |"#, "\n",
                          r#" \ V  V /  | | | | |  __/ | |_| |"# , "\n",
                          r#"  \_/\_/   |_| |_|  \___|  \__, |"#, "\n",
                          r#"                           |___/"#, "\n"
    ),
    arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Decorate {
        #[arg(short, long)]
        input: String,

        // #[arg(short, long, default_value = "tsv")]
        // format: String,
    },
}
use owo_colors::{OwoColorize,AnsiColors};
use crate::utils::utils::read_fasta;
use crate::config::Config;
use crate::config::PropertyColors;

// pub enum Decoration {
//     Property(PropertyScheme),
//     // Domains(Vec<Domain>),
// }

// pub enum PropertyScheme {
//     Physicochemical,
// }


pub struct PropertyPalette {
    pub hydrophobic: AnsiColors,
    pub positive: AnsiColors,
    pub negative: AnsiColors,
    pub polar: AnsiColors,
    pub special: AnsiColors,
}

impl PropertyPalette {
    pub fn from_config(cfg: &PropertyColors) -> Self {
        Self {
            hydrophobic: parse_color(&cfg.hydrophobic),
            positive: parse_color(&cfg.positive),
            negative: parse_color(&cfg.negative),
            polar: parse_color(&cfg.polar),
            special: parse_color(&cfg.special),
        }
    }
}

fn parse_color(name: &str) -> AnsiColors {
    match name.to_lowercase().as_str() {
        "red" => AnsiColors::Red,
        "green" => AnsiColors::Green,
        "blue" => AnsiColors::Blue,
        "yellow" => AnsiColors::Yellow,
        "purple" | "magenta" => AnsiColors::BrightMagenta,
        "cyan" => AnsiColors::Cyan,
        _ => AnsiColors::White,
    }
}

pub fn render_sequence(
    seq: &[u8],
    palette: &PropertyPalette,
    width: usize,
) -> String {
    let mut out = String::new();

    for (i, &b) in seq.iter().enumerate() {
        // Quebra de linha
        if i > 0 && i % width == 0 {
            out.push('\n');
        }

        out.push_str(
            &color_aa(b as char, palette)
        );
    }

    out
}


fn color_aa(aa: char, p: &PropertyPalette) -> String {
    match aa {
        'A' | 'V' | 'I' | 'L' | 'M' | 'F' | 'W' | 'Y' =>
            aa.to_string().color(p.hydrophobic).to_string(),

        'K' | 'R' | 'H' =>
            aa.to_string().color(p.positive).to_string(),

        'D' | 'E' =>
            aa.to_string().color(p.negative).to_string(),

        'S' | 'T' | 'N' | 'Q' =>
            aa.to_string().color(p.polar).to_string(),

        'C' =>
            aa.to_string().color(p.special).to_string(),

        _ =>
            aa.to_string(),
    }
}

// pub fn render_sequence(seq: &[u8]) -> String {
//     seq.iter()
//         .map(|&b| color_aa(b as char))
//         .collect::<Vec<_>>()
//         .join("")
// }

// pub fn render(seq: &[u8], config: &Config) -> String {
//     let palette = PropertyPalette::from_config(&config.decorate.properties);
//     render_sequence(seq, &palette, config.decorate.display.line_width)
// }

pub fn run_decorate(path: &str, config: &Config,) {
    let records = read_fasta(path).expect("Error reading FASTA");

    // let decoration = Decoration::Property(PropertyScheme::Physicochemical);

    let palette = PropertyPalette::from_config(&config.decorate.properties);

    for rec in records {
        println!(">{}", rec.id);
        let rendered = render_sequence(
            &rec.seq,
            &palette,
            config.decorate.display.line_width
        );
        println!("{}", rendered);
        println!();
    }
}
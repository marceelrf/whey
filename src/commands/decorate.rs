use owo_colors::OwoColorize;
use crate::utils::utils::read_fasta;
pub enum Decoration {
    Property(PropertyScheme),
    // Domains(Vec<Domain>),
}

pub enum PropertyScheme {
    Physicochemical,
}

pub fn color_aa(aa: char) -> String {
    match aa {
        // Hidrofóbicos
        'A' | 'V' | 'I' | 'L' | 'M' | 'F' | 'W' | 'Y' =>
            aa.to_string().yellow().to_string(),

        // Positivos
        'K' | 'R' | 'H' =>
            aa.to_string().blue().to_string(),

        // Negativos
        'D' | 'E' =>
            aa.to_string().red().to_string(),

        // Polares
        'S' | 'T' | 'N' | 'Q' =>
            aa.to_string().green().to_string(),

        // Especial
        'C' =>
            aa.to_string().purple().to_string(),

        _ =>
            aa.to_string(),
    }
}

pub fn render_sequence(seq: &[u8]) -> String {
    seq.iter()
        .map(|&b| color_aa(b as char))
        .collect::<Vec<_>>()
        .join("")
}

pub fn render(seq: &[u8], decoration: &Decoration) -> String {
    match decoration {
        Decoration::Property(PropertyScheme::Physicochemical) => {
            render_sequence(seq)
        }
    }
}

pub fn run_decorate(path: &str) {
    let records = read_fasta(path).expect("Error reading FASTA");

    let decoration = Decoration::Property(PropertyScheme::Physicochemical);

    for rec in records {
        println!(">{}", rec.id);
        println!("{}", render(&rec.seq, &decoration));
        println!();
    }
}
// use bio::alphabets;
use bio::io::fasta;

pub fn read_fasta() {

    let path = "C:\\\\Users\\\\madra\\\\OneDrive\\\\Desktop\\\\ongoing\\\\Packages\\\\ARquivos_fasta\\\\fake.fasta";

    let reader = fasta::Reader::from_file(caminho).expect("Error!!!");

    for result in reader.records() {
        let record = result.expect("Erro ao ler o registro");

        // Acessar os dados
        println!("ID: {}", record.id());
        println!("Descrição: {:?}", record.desc());
        
        // A sequência vem como um array de bytes (u8)
        let seq = record.seq();
        println!("Tamanho da sequência: {}", seq.len());
        
        // Se precisar converter para String:
        // let seq_str = String::from_utf8_lossy(seq);
    }
}
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::io::{self, BufRead};


fn main() {
    if let Ok(lines) = read_lines("./elf_calories.csv") {
        for line in lines {
            if let Ok(line) = line {
                let res: Result<i32, ParseIntError> = line.trim().parse();
                // facendo cosi leggo tutte le righe del file
                // provo a convertirle in int32 e se mi da errore perche 
                // la riga e' vuota, so che l'elfo e' cambiato

                if let Ok(res) = res {
                    println!("{}", res)
                } else {
                    println!("cambio")
                }
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
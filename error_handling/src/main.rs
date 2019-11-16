use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("uwu.txt").expect("Que macana, no se pudo abrir el archivo");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("uwu.txt") {
    //             Ok(f) => f,
    //             Err(e) => {
    //                 panic!("Macana creando el archivo: {:?}",e)
    //             }
    //         }
    //     },
    //     Err(err) => {
    //         panic!("Se mando una macana abriendo el archivo, {:?}",err);
    //     },
    // };
}

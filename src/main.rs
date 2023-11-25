use std::{fs::{self}, time::SystemTime};

struct Cli{
    pattern: String,
    path: String,
}

fn main() {
    let now = SystemTime::now();

    let pattern = std::env::args().nth(1).expect("Introduzca un commando");
    let path = std::env::args().nth(2).expect("Introduzca una ruta");

    let args = Cli{
        pattern,
        path
    };

    if args.is_dir(){
        args.remove_dir()
    }else if args.is_file(){
        args.remove_file()
    }else{
        args.not_remove()
    }

    println!("Estimado: {}ms", now.elapsed().unwrap().as_millis())
}


pub trait Delete{
    fn is_file(&self) -> bool;

    fn is_dir(&self) -> bool;

    fn remove_file(&self);

    fn remove_dir(&self);

    fn not_remove(&self);
}

impl Delete for Cli{

    fn not_remove(&self) {
        println!("Error al intentar remover");
        println!("Ruta: {}", self.path);
        println!("Comando: {}", self.pattern);
    }

    fn remove_file(&self){
        match fs::remove_file(&self.path){
            Ok(_) => println!("Archivo removido: {}", self.path),
            Err(e) => println!("Archivo error: {} path: {}", e, self.path)
        }
    }
    
    fn remove_dir(&self){
        match fs::remove_dir_all(&self.path){
            Ok(_) => println!("Folder Removido: {}", self.path),
            Err(e) => println!("Folder error: {} path: {}", e, self.path)
        }
    }
 
    fn is_file(&self) -> bool {
        if self.pattern == "-f"{
            return true
        }
        false
    }

    fn is_dir(&self) -> bool {
        if self.pattern == "-d"{
            return true
        }
        false
    }
}



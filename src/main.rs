use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::process::Command;

fn crear_limpiador_disco() -> std::io::Result<()>{
    const CONTENIDO: &str = "cleanmgr /sagerun";
    let mut archivo = File::create("C:\\rocotimizacion\\LimpiadorDisco.bat")?;
    archivo.write_all(CONTENIDO.as_bytes())?;
    Ok(())
}

fn crear_activador_office() -> std::io::Result<()>{
    const CONTENIDO: &str = "@echo off
cd 'C:\\Program Files (x86)\\Microsoft Office\\Office16'
echo EJECUTANDO ACTIVADOR
start '' 'OSPPREARM.exe'
ECHO PARTE 1: OK
timeout /t 2 /nobreak
start '' 'OSPPREARM.exe'
ECHO PARTE 2: OK
echo --- Los comandos se han ejecutado. Por favor, verifica el resultado. ---";
    let mut archivo = File::create("C:\\rocotimizacion\\ActivadorOffice.bat")?;
    archivo.write_all(CONTENIDO.as_bytes())?;
    Ok(())
}

fn crear_tarea(path: &str, nombre: &str){
    if Command::new("schtasks")
        .args(["/create", "/sc", "onstart", "/tn", nombre, "/tr", path])
        .output()
        .is_err() 
    {
        println!("Error al programar la tarea.");
    } else {
        println!("Se ha configurado al inicio");
        println!("--- {} completado ---",nombre);
    }
}
fn main(){
    let path = Path::new("C:\\rocotimizacion"); 
    match fs::create_dir_all(path) {
        Ok(_) => println!("Carpeta creada exitosamente!"),
        Err(e) => eprintln!("Error al crear la carpeta: {}", e),
    }
    
    match crear_limpiador_disco(){
        Ok(_) => println!("Se ha creado el archivo LimpiadorDisco.bat"),
        Err(err) => println!("Error al crear el archivo LimpiadorDisco.bat: {}", err),
    }
    crear_tarea("C:\\rocotimizacion\\LimpiadorDisco.bat", "LimpiadorDisco");
    
    match crear_activador_office(){
        Ok(_) => println!("Se ha creado el archivo ActivadorOffice.bat"),
        Err(err) => println!("Error al crear el archivo ActivadorOffice.bat: {}", err),
    }
    
    crear_tarea("C:\\rocotimizacion\\ActivadorOffice.bat", "ActivadorOffice");
    
    
}
use clap::Parser;
use std::{env, path::PathBuf, process::Command};
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(name = "vineflower-wrapper")]
#[command(about = "Decompile Java Files/Classes with Vineflower", long_about = None)]
struct Args {
    input: String,
    
    #[arg(short, long, default_value = "output")]
    output: String,
}

const VINEFLOWER_JAR: &[u8] = include_bytes!("../assets/vineflower.jar");

fn extract_jar() -> PathBuf {
    let temp_path = env::temp_dir().join("vineflower_temp.jar");
    let mut f = File::create(&temp_path).expect("Failed to create temp file");
    f.write_all(&VINEFLOWER_JAR).expect("Failed to write temp file");
    temp_path
}

fn main() {
    let args = Args::parse();
    let jar_path = extract_jar();
    
    let status = Command::new("java")
        .arg("-jar")
        .arg(&jar_path)
        .arg(&args.input)
        .arg(&args.output)
        .status()
        .expect("failed to execute process");
    
    if status.success() { 
        println!("Successfully decompiled Java Class/File saved into {}", args.output);
    } else {
        eprintln!("Failed to decompile Java Class/File");
    }
}

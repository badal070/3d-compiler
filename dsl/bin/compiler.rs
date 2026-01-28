use dsl_compiler::Compiler;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: dsl-compiler <file.dsl> [--optimize] [--validate-only] [--json]");
        process::exit(1);
    }
    
    let file_path = PathBuf::from(&args[1]);
    let mut _optimize = false;
    let mut validate_only = false;
    let mut output_json = false;
    
    // Parse flags
    for arg in &args[2..] {
        match arg.as_str() {
            "--optimize" => _optimize = true,
            "--validate-only" => validate_only = true,
            "--json" => output_json = true,
            _ => {}
        }
    }
    
    // Check file exists
    if !file_path.exists() {
        eprintln!("Error: File not found: {}", file_path.display());
        process::exit(1);
    }
    
    // Read DSL source
    let source = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    // Validate only mode
    if validate_only {
        match Compiler::default().compile(&source, file_path) {
            Ok(_) => {
                println!("✓ Validation successful");
                process::exit(0);
            }
            Err(errors) => {
                eprintln!("✗ Validation failed:");
                for error in errors {
                    eprintln!("  - {}", error);
                }
                process::exit(1);
            }
        }
    }
    
    // Compile
    match Compiler::default().compile(&source, file_path) {
        Ok(ir_scene) => {
            if output_json {
                // Output as JSON
                match serde_json::to_string(&ir_scene) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("Error serializing to JSON: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                // Pretty print IR
                println!("✓ Compilation successful!");
                println!("Scene: {}", ir_scene.metadata.name);
                println!("Entities: {}", ir_scene.entities.len());
                println!("Constraints: {}", ir_scene.constraints.len());
                println!("Motions: {}", ir_scene.motions.len());
                println!("Timelines: {}", ir_scene.timelines.len());
            }
            process::exit(0);
        }
        Err(errors) => {
            eprintln!("✗ Compilation failed:");
            for error in errors {
                eprintln!("  - {}", error);
            }
            process::exit(1);
        }
    }
}
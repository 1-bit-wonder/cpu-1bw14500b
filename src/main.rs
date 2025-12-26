use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;

mod cpu;

#[derive(Debug)]
struct Program {
    name: String,
    path: String,
}

fn main() -> std::io::Result<()> {
    let programs_dir = "programs";
    let mut programs = Vec::new();

    let entries = match fs::read_dir(programs_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading programs directory '{}': {}", programs_dir, e);
            eprintln!("Please ensure the directory exists and contains program files.");
            std::process::exit(1);
        }
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unnamed Program")
                        .to_string();
                    programs.push(Program {
                        name,
                        path: path.to_str().unwrap().to_string(),
                    });
                }
            }
        }
    }

    if programs.is_empty() {
        eprintln!("No programs found in the '{}' directory.", programs_dir);
        std::process::exit(1);
    }

    let selection_items: Vec<&String> = programs.iter().map(|p| &p.name).collect();
    let default_selection = programs.iter().position(|p| p.name == "blink").unwrap_or(0);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a program to run")
        .default(default_selection)
        .items(&selection_items)
        .interact_opt()
        .unwrap();

    if let Some(index) = selection {
        let chosen_program = &programs[index];

        let cycles: u32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("How many cycles to run?")
            .default(10)
            .interact_text()?;

        println!(
            "Running program {} from '{}' for {} cycles...",
            style(&chosen_program.name).cyan(),
            style(&chosen_program.path).italic(),
            style(cycles).bold()
        );

        let program_opcodes = match cpu::program_loader::load_program(&chosen_program.path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to load program: {}", e);
                std::process::exit(1);
            }
        };

        let outputs = cpu::run_program(program_opcodes, cycles);
        println!("Program finished.");
        println!("Outputs: {:?}", outputs);
    } else {
        println!("No program selected.");
    }

    Ok(())
}

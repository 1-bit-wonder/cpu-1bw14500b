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

    for entry in fs::read_dir(programs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
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

    if programs.is_empty() {
        eprintln!("No programs found in the '{}' directory.", programs_dir);
        return Ok(());
    }

    let selection_items: Vec<&String> = programs.iter().map(|p| &p.name).collect();
    let default_selection = programs.iter().position(|p| p.name == "blink").unwrap_or(0);

    if let Some(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a program to run")
        .default(default_selection)
        .items(&selection_items)
        .interact_opt()?
    {
        let chosen_program = &programs[index];

        let cycles: u32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("How many cycles to run?")
            .default(10)
            .interact_text()?;

        println!(
            "\nRunning program {} from '{}' for {} cycles...",
            style(&chosen_program.name).cyan(),
            style(&chosen_program.path).italic(),
            style(cycles).bold()
        );

        let program_opcodes = cpu::program_loader::load_program(&chosen_program.path)?;
        let mut cpu = cpu::Cpu::new(program_opcodes);

        // Manually enable IEN and OEN for the simulation
        cpu.lu.ien = true;
        cpu.lu.oen = true;

        let mut outputs = Vec::new();

        println!("\n{}", style("--- Execution Trace ---").bold().yellow());

        let bool_style = |b: bool| {
            if b {
                style(b).green()
            } else {
                style(b).red()
            }
        };

        for cycle in 0..cycles {
            let pc = cpu.pc;
            let opcode = cpu.program[pc as usize];
            let rr = cpu.lu.result_reg;
            let ien = cpu.lu.ien;
            let oen = cpu.lu.oen;
            let data = false; // Data bus is always false in this simulation

            let status = if let Some(output) = cpu.step(data) {
                outputs.push(output);
                style(format!("-> OUTPUT: {}", output))
                    .magenta()
                    .to_string()
            } else {
                "".to_string()
            };

            println!(
                "Cycle {:<2} | PC: {:<2} | Opcode: {:<4} | Data: {} | RR: {} | IEN: {} | OEN: {} | {}",
                cycle,
                style(pc).bold(),
                style(format!("{:?}", opcode)).cyan(),
                bool_style(data),
                bool_style(rr),
                bool_style(ien),
                bool_style(oen),
                status
            );
        }

        println!("\n{}", style("--- Final Results ---").bold().yellow());
        println!("Outputs: {:?}", outputs);
    } else {
        println!("No program selected.");
    }

    Ok(())
}

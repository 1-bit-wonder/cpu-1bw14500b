//! Utilities for loading MC14500B programs from the filesystem.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

use super::opcode::Opcode;

/// Loads a program from a text file.
///
/// The program file should contain one opcode mnemonic per line.
/// Empty lines are ignored, and parsing is case-insensitive.
///
/// # Arguments
///
/// * `file_path` - The path to the program file.
///
/// # Returns
///
/// A `Result` containing either the loaded `Vec<Opcode>` on success,
/// or an `io::Error` on failure. An error can occur if the file cannot be
/// opened, or if it contains an invalid opcode string.
pub fn load_program(file_path: &str) -> io::Result<Vec<Opcode>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut program = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if !line.trim().is_empty() {
            let opcode = Opcode::from_str(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            program.push(opcode);
        }
    }

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_program_ok() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "LD\nLDC\nSTO").unwrap();

        let program = load_program(file.path().to_str().unwrap()).unwrap();
        assert_eq!(program, vec![Opcode::LD, Opcode::LDC, Opcode::STO]);
    }

    #[test]
    fn test_load_program_with_empty_lines() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "LD\n\nLDC\nSTO\n").unwrap();

        let program = load_program(file.path().to_str().unwrap()).unwrap();
        assert_eq!(program, vec![Opcode::LD, Opcode::LDC, Opcode::STO]);
    }

    #[test]
    fn test_load_program_invalid_opcode() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "LD\nINVALID\nSTO").unwrap();

        let result = load_program(file.path().to_str().unwrap());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::InvalidData);
    }
}

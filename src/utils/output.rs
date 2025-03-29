use colored::*;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::time::Duration;

/// Print a message in rainbow colors
pub fn print_rainbow(message: &str) {
    let colors = [
        "red", "yellow", "green", "cyan", "blue", "magenta",
    ];
    
    for (i, c) in message.chars().enumerate() {
        let color_index = i % colors.len();
        let color = colors[color_index];
        
        let colored_char = match color {
            "red" => c.to_string().red(),
            "yellow" => c.to_string().yellow(),
            "green" => c.to_string().green(),
            "cyan" => c.to_string().cyan(),
            "blue" => c.to_string().blue(),
            "magenta" => c.to_string().magenta(),
            _ => c.to_string().normal(),
        };
        
        print!("{}", colored_char);
    }
    println!();
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "ERROR:".bold().red(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    eprintln!("{} {}", "WARNING:".bold().yellow(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", "INFO:".bold().blue(), message);
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "SUCCESS:".bold().green(), message);
}

/// Create a progress bar with a spinner
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Print a prompt and get user input
pub fn prompt(message: &str) -> io::Result<String> {
    print!("{} ", message.bold());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Trim trailing newline
    if input.ends_with('\n') {
        input.pop();
    }
    if input.ends_with('\r') {
        input.pop();
    }
    
    Ok(input)
}

/// Clear the screen
pub fn clear_screen() -> io::Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    Ok(())
}

/// Print a table with headers and rows
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    // Find the maximum width for each column
    let mut col_widths = vec![0; headers.len()];
    
    // Consider headers for column width
    for (i, header) in headers.iter().enumerate() {
        col_widths[i] = header.len();
    }
    
    // Consider data for column width
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(headers.len()) {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }
    
    // Print headers
    for (i, header) in headers.iter().enumerate() {
        print!("| {:<width$} ", header.bold(), width = col_widths[i]);
    }
    println!("|");
    
    // Print separator
    for width in &col_widths {
        print!("+{:-<width$}-", "", width = width);
    }
    println!("+");
    
    // Print rows
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(headers.len()) {
            print!("| {:<width$} ", cell, width = col_widths[i]);
        }
        println!("|");
    }
} 
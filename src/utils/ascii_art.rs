use colored::*;
use figlet_rs::FIGfont;
use terminal_size::{Width, Height, terminal_size};

/// Print LlamaClick ASCII art banner
pub fn print_llama_ascii_art() {
    // Load font
    let standard_font = FIGfont::standard().unwrap();
    
    // Generate figlet text
    let figlet = standard_font.convert("LlamaClick").unwrap();
    
    // Print with rainbow colors
    let figlet_lines: Vec<&str> = figlet.to_string().lines().collect();
    for line in figlet_lines {
        print_rainbow_figlet_line(line);
    }
    
    // Print the llama ASCII art
    print_llama_image();
}

/// Print a figlet line with rainbow colors
fn print_rainbow_figlet_line(line: &str) {
    let colors = [
        "red", "yellow", "green", "cyan", "blue", "magenta",
    ];
    
    for (i, c) in line.chars().enumerate() {
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

/// Print a small llama ASCII art
fn print_llama_image() {
    let llama = r#"
                  ,,__
        ..  ..   / o._)                   .---.
       /--'/--\  \-'||        .----.    .'     '.
      /        \_/ / |      .'      '..'         '-.
    .'\  \__\  __.'.'     .'          i-._
      )\ |  )\ |      _.'
     // \\ // \\
    ||_  \\|_  \\_
    '--' '--'' '--' 
    "#;
    
    println!("{}", llama.bright_yellow());
}

/// Get terminal dimensions
pub fn get_terminal_dimensions() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (80, 24) // Default fallback
    }
}

/// Center a string in the terminal
pub fn center_string(s: &str, width: usize) -> String {
    let padding = (width.saturating_sub(s.len())) / 2;
    format!("{:>width$}", s, width = s.len() + padding)
} 
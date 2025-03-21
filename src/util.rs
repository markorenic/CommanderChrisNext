use crate::error::Result;
use std::io::{self, Write};
use std::process::Command;

/// Analyzes a shell command for potential risks
fn analyze_command_safety(cmd: &str) -> (bool, Vec<&str>) {
    let dangerous_commands = [
        "rm", "sudo", "mv", "dd", ">", "mkfs", "chmod", "chown", "kill",
    ];
    let side_effects = [
        "write",
        "create",
        "delete",
        "modify",
        "install",
        "uninstall",
        "download",
    ];

    let mut is_dangerous = false;
    let mut effects = Vec::new();

    // Check for dangerous commands
    for &dangerous in dangerous_commands.iter() {
        if cmd.contains(dangerous) {
            is_dangerous = true;
            break;
        }
    }

    // Identify potential side effects
    for &effect in side_effects.iter() {
        if cmd.to_lowercase().contains(effect) {
            effects.push(effect);
        }
    }

    (is_dangerous, effects)
}

/// Execute a shell command and return its output
pub fn execute_command(cmd: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", cmd]).output()
    } else {
        Command::new("sh").args(["-c", cmd]).output()
    }
    .map_err(|e| crate::error::AppError::Unknown(format!("Failed to execute command: {}", e)))?;

    let mut result = String::new();
    if !output.stdout.is_empty() {
        result.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        if !result.is_empty() {
            result.push_str("\n");
        }
        result.push_str(&String::from_utf8_lossy(&output.stderr));
    }

    Ok(result)
}

/// Prints a styled header to the terminal
pub fn print_header(text: &str) {
    let terminal_width = terminal_size().unwrap_or(80);
    let padding = "=".repeat((terminal_width - text.len() - 2) / 2);

    println!("\n{} {} {}\n", padding, text, padding);
}

/// Gets the terminal size (width) if possible
pub fn terminal_size() -> Option<usize> {
    if let Some((width, _)) = term_size::dimensions() {
        Some(width)
    } else {
        None
    }
}

/// Prompts the user for input
pub fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// Prompts the user for a yes/no answer
pub fn prompt_yes_no(prompt: &str, default: bool) -> Result<bool> {
    let prompt_with_default = if default {
        format!("{} [Y/n]: ", prompt)
    } else {
        format!("{} [y/N]: ", prompt)
    };

    print!("{}", prompt_with_default);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Ok(default);
    }

    match input.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => {
            println!(
                "Invalid input, using default: {}",
                if default { "yes" } else { "no" }
            );
            Ok(default)
        }
    }
}

/// Extract shell commands from a response string
pub fn extract_commands(response: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut current_command = String::new();

    for line in response.lines() {
        if line.starts_with("```") {
            if !in_code_block {
                // Start of code block
                in_code_block = true;
                code_block_lang = line.trim_start_matches('`').to_string();
                current_command.clear();
            } else {
                // End of code block
                in_code_block = false;
                if code_block_lang.contains("sh")
                    || code_block_lang.contains("bash")
                    || code_block_lang.contains("shell")
                {
                    if !current_command.trim().is_empty() {
                        commands.push(current_command.trim().to_string());
                    }
                }
            }
        } else if in_code_block {
            current_command.push_str(line);
            current_command.push('\n');
        }
    }

    commands
}

/// Formats a response with optional highlighting for code blocks
pub fn format_response(response: &str) -> String {
    let mut formatted = String::new();
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_lang = String::new();

    // Format the response
    for line in response.lines() {
        if line.starts_with("```") {
            if !in_code_block {
                // Start of code block
                in_code_block = true;
                code_block_lang = line.trim_start_matches('`').to_string();
                code_block_content.clear();
            } else {
                // End of code block
                in_code_block = false;

                // Format the code block
                formatted.push_str("┌── ");
                formatted.push_str(&code_block_lang);
                formatted.push_str(" ");
                formatted.push_str(&"─".repeat(80 - code_block_lang.len() - 6));
                formatted.push('\n');

                for content_line in code_block_content.lines() {
                    formatted.push_str("│ ");
                    formatted.push_str(content_line);
                    formatted.push('\n');
                }

                formatted.push_str("└");
                formatted.push_str(&"─".repeat(80));
                formatted.push('\n');
            }
        } else if in_code_block {
            code_block_content.push_str(line);
            code_block_content.push('\n');
        } else {
            formatted.push_str(line);
            formatted.push('\n');
        }
    }

    formatted
}

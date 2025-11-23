use colored::Colorize;

pub fn success(text: &str) -> String {
    text.green().to_string()
}

pub fn error(text: &str) -> String {
    text.red().to_string()
}

pub fn warning(text: &str) -> String {
    text.yellow().to_string()
}

pub fn info(text: &str) -> String {
    text.cyan().to_string()
}

pub fn highlight(text: &str) -> String {
    text.bright_blue().bold().to_string()
}

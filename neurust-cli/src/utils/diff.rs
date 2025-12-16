use colored::Colorize;
use console::{Style, Term};
use similar::{ChangeTag, TextDiff};
use std::io::Write;

// 🔥 New Enum for User Choice
pub enum ConfirmAction {
    Yes,
    No,
    All, // "Always allow for this session"
}

pub fn show_diff_and_confirm(
    file_path: &str,
    old_content: &str,
    new_content: &str,
    reason: &str,
) -> ConfirmAction {
    let diff = TextDiff::from_lines(old_content, new_content);
    let mut changes_found = false;

    // 1. Header with Reason
    let title = if old_content.is_empty() {
        format!("✨ NEW FILE: {}", file_path)
    } else {
        format!("📝 EDITING: {}", file_path)
    };

    println!("\n{}", Style::new().cyan().bold().apply_to(title));
    if !reason.is_empty() {
        println!(
            "💡 Reason: {}",
            Style::new().yellow().italic().apply_to(reason)
        );
    }
    println!("---------------------------------------------------");

    // 2. Diff View (Same as before)
    for change in diff.iter_all_changes() {
        let (sign, style) = match change.tag() {
            ChangeTag::Delete => ("-", Style::new().red()),
            ChangeTag::Insert => ("+", Style::new().green()),
            ChangeTag::Equal => (" ", Style::new().dim()),
        };
        if change.tag() != ChangeTag::Equal {
            changes_found = true;
        }
        print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
    }
    println!("---------------------------------------------------");

    if !changes_found {
        return ConfirmAction::Yes;
    }

    // 3. Improved Confirmation Loop
    loop {
        // Option တွေကို ရှင်းရှင်းလင်းလင်း ပြမယ်
        println!(
            "{}",
            "Options: [y]es, [n]o, [a]ll (accept all changes)".blue()
        );
        print!("{} Apply changes? > ", "❓".yellow());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return ConfirmAction::Yes,
            "n" | "no" => {
                println!("❌ Skipped: {}", file_path);
                return ConfirmAction::No;
            }
            "a" | "all" => {
                println!("🚀 Accepting ALL future changes for this session!",);
                return ConfirmAction::All;
            }
            _ => println!("Please type 'y', 'n', or 'a'."),
        }
    }
}

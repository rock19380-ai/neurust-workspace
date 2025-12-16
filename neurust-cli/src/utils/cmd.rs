use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::{Result, anyhow, Context};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{theme::ColorfulTheme, Select};

/// အချိန်ကြာမြင့်နိုင်သော Command များစာရင်း
const HEAVY_COMMANDS: &[&str] = &["npx", "npm", "cargo", "pnpm", "yarn", "docker", "git"];

/// 1. Interactive Execution (Console Output Only)
/// User မြင်အောင် Console မှာ ပြမယ်၊ Heavy command ဆိုရင် မေးမယ်။
pub fn execute_with_output(program: &str, args: &[&str], cwd: Option<&str>) -> Result<()> {
    // Heavy Command ဟုတ်မဟုတ် စစ်ဆေးခြင်း
    if HEAVY_COMMANDS.contains(&program) {
        return execute_heavy_interactive(program, args, cwd);
    }
    // Light Command ဆိုရင် ပုံမှန်အတိုင်း run မယ်
    execute_silent(program, args, cwd)
}

/// 2. Capture Execution (Returns String) - 🔥 NEW FUNCTION
/// Console မှာ မပြဘဲ Output ကို Variable ထဲ ထည့်ချင်တဲ့အခါ သုံးမယ် (e.g. Audit JSON)
pub fn execute_and_capture(program: &str, args: &[&str], cwd: Option<&str>) -> Result<String> {
    let mut cmd = Command::new(program);
    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    // Output ကို ဖမ်းယူခြင်း
    let output = cmd.output().context("Failed to execute capture command")?;

    // Success ဖြစ်ဖြစ် Fail ဖြစ်ဖြစ် Output ကို String ပြောင်းမယ်
    // (ဥပမာ cargo audit က အမှားတွေ့ရင် exit code 1 ပြန်ပေမယ့် JSON လိုချင်သေးတယ်)
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    if stdout.trim().is_empty() {
        // Stdout မရှိရင် Stderr ကို ပြန်ပေးမယ်
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if !stderr.is_empty() {
            return Ok(stderr);
        }
    }

    Ok(stdout)
}

// --- Internal Helpers ---

fn execute_heavy_interactive(program: &str, args: &[&str], cwd: Option<&str>) -> Result<()> {
    println!("\n{} Heavy Task Detected: {} {}", "⚠️".yellow(), program, args.join(" "));
    
    if let Some(dir) = cwd {
        println!("   📂 In Directory: {}", dir);
    }

    println!("{}", "This command requires downloading packages from the internet and may take a while.".dimmed());

    let selections = &[
        "⏳ Wait (Neurust will run it patiently with a spinner)",
        "✋ Skip (I will run it manually later)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How do you want to proceed?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap_or(0);

    if selection == 1 {
        // Option B: Skip
        println!("\n{} Skipped. Please run this manually:", "⏭️".blue());
        let cd_cmd = if let Some(dir) = cwd { format!("cd {} && ", dir) } else { "".to_string() };
        println!("   {}{}{} {}\n", cd_cmd.cyan(), program.green(), args.join(" ").green(), "".clear());
        return Ok(()); 
    }

    // Option A: Wait -> Show Spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));
    
    pb.set_message(format!("Running {}... (Please be patient)", program));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(dir) = cwd { cmd.current_dir(dir); }

    // Piped Output to avoid messing up the spinner
    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output().context("Failed to spawn process")?;

    pb.finish_and_clear();

    if output.status.success() {
        println!("{} Command finished successfully.", "✅".green());
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        println!("{} Command failed:\n{}", "❌".red(), err_msg.red());
        Err(anyhow!("External command failed"))
    }
}

fn execute_silent(program: &str, args: &[&str], cwd: Option<&str>) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(dir) = cwd { cmd.current_dir(dir); }
    let output = cmd.output().context("Failed to execute command")?;
    if output.status.success() { Ok(()) } else { Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string())) }
}

/// Backward compatibility alias
pub fn execute(program: &str, args: &[&str], cwd: Option<&str>) -> Result<String> {
    execute_and_capture(program, args, cwd)
}
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor};
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow;
use std::path::PathBuf;
use crate::commands::ask;

// 1. Helper Struct
#[derive(Helper, Completer, Hinter, Validator)]
pub struct NeurustHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Ignore)] 
    highlighter: MatchingBracketHighlighter, 
}

// 2. Syntax Highlighting Implementation
impl Highlighter for NeurustHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        // Basic Syntax Highlighting Logic
        let highlighted = line
            .replace("create", &"create".green().bold().to_string())
            .replace("deploy", &"deploy".yellow().bold().to_string())
            .replace("audit", &"audit".red().bold().to_string())
            .replace("ask", &"ask".cyan().bold().to_string());
        
        // Bracket matching highlight ကိုပါ ပေါင်းစပ်မယ်
        // ဒီနေရာမှာ highlighted ကို borrow လုပ်ထားပါတယ်
        let final_cow = self.highlighter.highlight(&highlighted, pos);
        
        // 🔥 FIX: Borrow လုပ်ထားတာကို To String နဲ့ အသစ်ပွားပြီး Owned အဖြစ်ပြောင်းမယ်
        // ဒါမှ Function ပြီးသွားလည်း Return ပြန်လို့ရမှာပါ
        Cow::Owned(final_cow.to_string())
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

// 3. Main REPL Loop
pub async fn start() -> anyhow::Result<()> {
    println!("{}", "🚀 Neurust AI Shell (v2.0)".green().bold());
    println!("{}", "Tip: Use ⬆️/⬇️ for history. Type 'exit' to quit.".dimmed());
    println!("------------------------------------------------");

    // Config Setup
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .build();

    let h = NeurustHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter {},
        validator: MatchingBracketValidator::new(),
        highlighter: MatchingBracketHighlighter::new(),
    };

    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));

    // History Loading
    let history_path = dirs::home_dir()
        .map(|p| p.join(".neurust_history"))
        .unwrap_or_else(|| PathBuf::from(".neurust_history"));

    if rl.load_history(&history_path).is_err() {
        // No previous history
    }

    loop {
        let prompt = format!("{}", "neurust> ".cyan().bold());
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() { continue; }

                let _ = rl.add_history_entry(input);

                if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
                    println!("Saving session... Bye! 👋");
                    break;
                }

                println!(); 
                if let Err(e) = ask::execute(input.to_string()).await {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
                println!("------------------------------------------------");
            }
            Err(ReadlineError::Interrupted) => {
                println!("(CTRL-C) - Type 'exit' to quit");
            }
            Err(ReadlineError::Eof) => {
                println!("(CTRL-D) - Exiting");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    rl.save_history(&history_path)?;
    Ok(())
}
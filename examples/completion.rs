use dialoguer::{theme::ColorfulTheme, Completion, Input};

fn main() -> Result<(), std::io::Error> {
    println!("Use the Right arrow or Tab to complete your command");
    let completion = MyCompletion::default();
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("dialoguer")
        .completion_with(&completion)
        .interact_text()?;
    Ok(())
}

struct MyCompletion {
    options: Vec<String>,
}

impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: vec![
                "orange".to_string(),
                "apple".to_string(),
                "apppppple".to_string(),
                "banana".to_string(),
            ],
        }
    }
}

impl Completion for MyCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<CompletionResult> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(CompletionResult {
                completion: matches[0].to_string(),
                redraw_prompt: false,
            })
        } else {
            None
        }
    }
}

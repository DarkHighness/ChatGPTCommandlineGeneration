use std::fs;
use std::io::Write;

use api::Api;
#[cfg(feature = "readline")]
use rustyline::DefaultEditor;

mod api;
mod constant;
mod model;

#[cfg(feature = "readline")]
fn read_user_input() -> String {
    let mut buffer = String::new();
    let mut editor = DefaultEditor::new()?;

    loop {
        let line = editor.readline(">");

        match line {
            Ok(line) => {
                buffer += &line;
            }
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(_) => {
                std::process::exit(1);
            }
        }
    }

    return buffer;
}

#[cfg(not(feature = "readline"))]
fn read_user_input() -> String {
    use std::io::stdin;

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_retries = 5;

    let mut history_path = home::home_dir().expect("Unknown HOME_DIR");
    history_path.push(".cg.history");
    let history_path = history_path;

    let mut history_file = fs::File::options()
        .read(true)
        .create(true)
        .append(true)
        .open(&history_path)?;

    let api = Api::key_from_env();
    let user_input = read_user_input();

    writeln!(history_file, "{}", user_input)?;

    let mut retries = 0;

    let ret = loop {
        let command = api.get_commandline(&user_input).await;

        if let Ok(command) = command {
            println!("{}", command);
            writeln!(history_file, "{}", command)?;

            break Ok(());
        }

        retries += 1;

        eprintln!("Retries {}/{}", retries, max_retries);

        if retries >= max_retries {
            break command.map(|_| ());
        }
    };

    if let Err(err) = ret {
        eprint!("{:?}", err);
        std::process::exit(1);
    }

    Ok(())
}

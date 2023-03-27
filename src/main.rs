use std::process::{Child, Command};

use inquire::{Select, Text};

fn name_project() -> String {
    let name = Text::new("What is the name of your project?").prompt();

    match name {
        Ok(name) => name,
        Err(_) => {
            println!(
                "An error happened when asking for the name of your project, try again later."
            );
            std::process::exit(1);
        }
    }
}

fn select_language() -> String {
    let options = vec![
        "Rust",
        "Python",
        "TypeScript",
        "C++",
        "C",
        "C#",
        "Java",
        "Go",
    ];

    let ans = Select::new("What Programming language do you want to use?", options).prompt();

    match ans {
        Ok(choise) => choise.to_string(),
        Err(_) => {
            println!("Error");
            std::process::exit(1);
        }
    }
}
fn visibility_of_repo() -> bool {
    let ans = Select::new(
        "How to you want to make your repository public or private?",
        vec!["Private", "Public"],
    )
    .prompt();

    match ans {
        Ok(choise) => {
            if choise == "Public" {
                true
            } else {
                false
            }
        }
        Err(_) => {
            println!("Error");
            std::process::exit(1);
        }
    }
}

fn generate_git() -> bool {
    let ans = Select::new(
        "Do you want to generate a git repository?",
        vec!["Yes", "No"],
    )
    .prompt();

    match ans {
        Ok(choise) => {
            if choise == "Yes" {
                true
            } else {
                false
            }
        }
        Err(_) => {
            println!("Error");
            std::process::exit(1);
        }
    }
}

fn generate_project(name: String, language: String, is_public: bool) -> Child {
    match is_public {
        true => {
            return Command::new("gh")
                .arg("repo")
                .arg("create")
                .arg(name)
                .arg("-c")
                .arg("-g")
                .arg(language)
                .arg("--public")
                .spawn()
                .expect("failed to execute process");
        }
        false => {
            return Command::new("gh")
                .arg("repo")
                .arg("create")
                .arg(name)
                .arg("-c")
                .arg("-g")
                .arg(language)
                .arg("--private")
                .spawn()
                .expect("failed to execute process");
        }
    }
}

fn main() {
    let name = name_project();
    let language = select_language();
    generate_git();
    let is_public = visibility_of_repo();
    let out = generate_project(name, language, is_public);
    let test = out.stdout;
    println!("{:?}", test);
}

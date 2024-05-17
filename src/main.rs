use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{self, exit, Command, Stdio};
use std::time::Duration;
use std::{fs, io};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Verify all exercises according to the recommended order
    Verify,
    /// Rerun `verify` when files were edited
    Watch,
    /// Run/Test a single exercise
    Run {
        /// The name of the exercise
        name: String,
    },
}

const WELCOME: &str = "HI FROM NIXLINGS";

const DEFAULT_OUT: &str = "Thanks for installing Nixlings";

fn main() -> Result<()> {
    let args = Args::parse();

    if args.command.is_none() {
        println!("\n{WELCOME}\n");
    }

    // TODO change this to check to see if experimental features flakes are enabled
    if which::which("nix").is_err() {
        println!("We cannot find `nix`.");
        println!("Try running `nix --version` to diagnose your problem.");
        println!("For instructions on how to install Nix, check the README.");
        std::process::exit(1);
    }

    let info_file =
        fs::read_to_string("info.toml").unwrap_or_else(|e| {
            match e.kind() {
                io::ErrorKind::NotFound => println!(
                    "The program must be run from the nixlings directory\nTry `cd nixlings/`!",
                ),
                _ => println!("Failed to read the info.toml file: {e}"),
            }
            std::process::exit(1);
        });

    let exercises = toml_edit::de::from_str::<ExerciseList>(&info_file)
        .unwrap()
        .exercises;

    let command = args.command.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0)
    });

    match command {
        Subcommands::Watch => {}
        Subcommands::Run { name } => {
            let exercise = find_exercise(&name, &exercises);

            run(exercise).unwrap_or_else(|_| std::process::exit(1));
        }
        Subcommands::Verify => {}
    }

    Ok(())
}

pub fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    exercises
        .iter()
        .find(|e| e.name == name)
        .unwrap_or_else(|| {
            println!("No exercise found for '{name}'!");
            std::process::exit(1)
        })
}

//run.rs
pub fn run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = indicatif::ProgressBar::new_spinner();
    progress_bar.set_message(format!("running {}", exercise.name));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let flake_check_result = exercise.flake_check();
    Ok(())
}

//exercise.rs
#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// A representation of a rustlings exercise.
// This is deserialized from the accompanying info.toml file
#[derive(Deserialize, Debug)]
pub struct Exercise {
    // Name of the exercise
    pub name: String,
    // The path to the file containing the exercise's source code
    pub path: PathBuf,
    // The task at hand
    pub task: String,
}

#[derive(Deserialize, Debug)]
pub struct ExerciseOutput {
    stdout: String,
    stderr: String,
}

impl Exercise {
    pub fn flake_check(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let cmd = Command::new("nix flake check")
            .output()
            .expect("flake check failed to even check");

        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        };
        if cmd.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }
}

//impl std::fmt::Display for Exercise {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        format!(self.name)
//    }
//}

//use std::fs;
//use std::path::Path;
//use std::process::Command;
//
//struct Exercise {
//    name: String,
//    path: String,
//}
//
//impl Exercise {
//    fn new(name: String, path: String) -> Exercise {
//        Exercise { name, path }
//    }
//
//    fn is_done(&self) -> bool {
//        let flake_nix_path = Path::new(&self.path).join("flake.nix");
//        let contents = fs::read_to_string(&flake_nix_path).expect("Failed to read flake.nix file");
//        !contents.contains("# I AM NOT DONE")
//    }
//
//    fn run_checks(&self) -> bool {
//        let output = Command::new("nix")
//            .arg("flake")
//            .arg("check")
//            .arg(&self.path)
//            .output()
//            .expect("Failed to run Nix flake check");
//
//        output.status.success()
//    }
//}

//fn main() {
//    let exercises_dir = "exercises";
//    let exercises = find_exercises(exercises_dir);
//
//    let mut completed_exercises = 0;
//    let mut passed_exercises = 0;
//
//    for exercise in &exercises {
//        println!("Checking exercise: {}", exercise.name);
//
//        if exercise.is_done() {
//            completed_exercises += 1;
//            println!("✅ {} is completed!", exercise.name);
//
//            println!("Running checks for exercise: {}", exercise.name);
//            if exercise.run_checks() {
//                passed_exercises += 1;
//                println!("✅ {} passed the checks!", exercise.name);
//            } else {
//                println!("❌ {} did not pass the checks.", exercise.name);
//            }
//        } else {
//            println!("❌ {} is not yet completed.", exercise.name);
//        }
//
//        println!();
//    }
//
//    println!(
//        "Progress: You have completed {} out of {} exercises!",
//        completed_exercises,
//        exercises.len()
//    );
//    println!(
//        "Checks: {} out of {} exercises passed the checks!",
//        passed_exercises, completed_exercises
//    );
//}

//fn find_exercises(directory: &str) -> Vec<Exercise> {
//    let mut exercises = Vec::new();
//    println!("here1");
//    for entry in fs::read_dir(directory).expect("Failed to read exercises directory") {
//        println!("here");
//        let entry = entry.expect("Failed to read directory entry");
//        let path = entry.path();
//        if path.is_dir() {
//            let flake_nix_path = path.join("flake.nix");
//            if flake_nix_path.exists() {
//                let name = path.file_name().unwrap().to_str().unwrap().to_string();
//                let path_str = path.to_str().unwrap().to_string();
//                exercises.push(Exercise::new(name, path_str));
//            }
//        }
//    }
//    exercises
//}

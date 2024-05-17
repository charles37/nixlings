use std::fs;
use std::path::Path;
use std::process::Command;

struct Exercise {
    name: String,
    path: String,
}

impl Exercise {
    fn new(name: String, path: String) -> Exercise {
        Exercise { name, path }
    }

    fn is_done(&self) -> bool {
        let flake_nix_path = Path::new(&self.path).join("flake.nix");
        let contents = fs::read_to_string(&flake_nix_path).expect("Failed to read flake.nix file");
        !contents.contains("# I AM NOT DONE")
    }

    fn run_checks(&self) -> bool {
        let output = Command::new("nix")
            .arg("flake")
            .arg("check")
            .arg(&self.path)
            .output()
            .expect("Failed to run Nix flake check");

        output.status.success()
    }
}

fn main() {
    let exercises_dir = "exercises";
    let exercises = find_exercises(exercises_dir);

    let mut completed_exercises = 0;
    let mut passed_exercises = 0;

    for exercise in &exercises {
        println!("Checking exercise: {}", exercise.name);

        if exercise.is_done() {
            completed_exercises += 1;
            println!("✅ {} is completed!", exercise.name);

            println!("Running checks for exercise: {}", exercise.name);
            if exercise.run_checks() {
                passed_exercises += 1;
                println!("✅ {} passed the checks!", exercise.name);
            } else {
                println!("❌ {} did not pass the checks.", exercise.name);
            }
        } else {
            println!("❌ {} is not yet completed.", exercise.name);
        }

        println!();
    }

    println!(
        "Progress: You have completed {} out of {} exercises!",
        completed_exercises,
        exercises.len()
    );
    println!(
        "Checks: {} out of {} exercises passed the checks!",
        passed_exercises, completed_exercises
    );
}

fn find_exercises(directory: &str) -> Vec<Exercise> {
    let mut exercises = Vec::new();
    println!("here1");
    for entry in fs::read_dir(directory).expect("Failed to read exercises directory") {
        println!("here");
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            let flake_nix_path = path.join("flake.nix");
            if flake_nix_path.exists() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let path_str = path.to_str().unwrap().to_string();
                exercises.push(Exercise::new(name, path_str));
            }
        }
    }
    exercises
}

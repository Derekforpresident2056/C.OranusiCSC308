use std::process::Command;

fn main() {
    
    let run_echo = || {
        let output = Command::new("echo")
            .arg("Hello from child process!")
            .output()
            .expect("Failed to execute command");

        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let result = run_echo();
    println!("Child process output: {}", result);
}
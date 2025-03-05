use std::process::Command;
use std::env;
use which::which;

fn main() {
    // Get project name from the user
    let project_name = env::args().nth(1).unwrap_or_else(|| "my-vite-app".to_string());

    println!("🚀 Setting up Vite + React + Tailwind v4 in /{}/...", project_name);

    // Check if npm is installed
    if which("npm").is_err() {
        eprintln!("❌ npm is not installed. Please install Node.js first.");
        return;
    }

    // List of ternminal commands to run
    let commands = [
     format!("npm create vite@latest {} -- --template react", project_name),
     format!("cd {} && npm install", project_name),
     format!("cd {} && npm install tailwindcss @tailwindcss/vite", project_name),
    ];

    // Run each command in the list
    for cmd in &commands {
        println!("🔧 Running: {}", cmd);
        let _ = Command::new("sh").arg("-c").arg(cmd).status();
    };

    println!("\n✅ Setup complete! Run:");
    println!("👉 cd {} && npm run dev", project_name);
}
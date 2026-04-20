use clap::{Parser, Subcommand};
use inquire::{Text, Select, MultiSelect, Confirm};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(name = "neurogenesis")]
#[command(about = "The Day 0 Cognitive Bootstrapper for Neuro Agentic AI OS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new NeuroGenesis project with a Socratic interview
    Init,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiskProfile {
    level: String,
    compliance_requirements: Vec<String>,
    security_posture: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TargetAudience {
    primary_user: String,
    scale_expectation: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TechStack {
    frontend: String,
    backend: String,
    database: String,
    infrastructure: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CriticalArea {
    area: String,
    trigger_condition: String,
    required_specialist: String,
    rationale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NeuroOsDirectives {
    required_panels: Vec<String>,
    required_specialists: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenesisContext {
    project_name: String,
    domain: String,
    risk_profile: RiskProfile,
    user_emphasized_priorities: Vec<String>,
    target_audience: TargetAudience,
    tech_stack: TechStack,
    core_constraints: Vec<String>,
    critical_areas: Vec<CriticalArea>,
    neuro_os_directives: NeuroOsDirectives,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("🧠 Welcome to the NeuroGenesis Socratic Inquisitor.");
            println!("Let's explore the true nature of your project.\n");

            // 1. Project Name
            let project_name = Text::new("What is the name of this project?")
                .with_default("my-neuro-project")
                .prompt()
                .unwrap();

            // 2. Domain
            let domain = Text::new("What is the primary domain or industry? (e.g., FinTech, E-commerce, Internal Tool)")
                .prompt()
                .unwrap();

            // 3. Risk Profile
            let risk_options = vec!["Low (Internal/Prototype)", "Medium (B2B SaaS)", "High (Healthcare/PII)", "Extreme (FinTech/Defense)"];
            let risk_level = Select::new("What is the risk profile of this system?", risk_options)
                .prompt()
                .unwrap()
                .to_string();

            // 4. Tech Stack
            println!("\nLet's discuss the Tech Stack.");
            let frontend = Text::new("Frontend technology? (Leave blank for None)")
                .prompt()
                .unwrap();
            
            let backend = Text::new("Backend technology?")
                .prompt()
                .unwrap();

            let database = Text::new("Database / State Management?")
                .prompt()
                .unwrap();

            let infrastructure = Text::new("Infrastructure / Deployment target?")
                .with_default("Docker/Kubernetes")
                .prompt()
                .unwrap();

            // 5. Priorities
            let priorities_input = Text::new("What are the absolute non-negotiable priorities? (Comma separated)")
                .prompt()
                .unwrap();
            let priorities: Vec<String> = priorities_input.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

            // 6. Target Audience
            let target_user = Text::new("Who is the primary user?")
                .prompt()
                .unwrap();
            
            let scale = Text::new("What is the expected scale or concurrency?")
                .with_default("100s of users")
                .prompt()
                .unwrap();

            // Generate the struct
            let context = GenesisContext {
                project_name: project_name.clone(),
                domain,
                risk_profile: RiskProfile {
                    level: risk_level,
                    compliance_requirements: vec![],
                    security_posture: "Standard".to_string(),
                },
                user_emphasized_priorities: priorities,
                target_audience: TargetAudience {
                    primary_user: target_user,
                    scale_expectation: scale,
                },
                tech_stack: TechStack {
                    frontend,
                    backend,
                    database,
                    infrastructure,
                },
                core_constraints: vec![],
                critical_areas: vec![],
                neuro_os_directives: NeuroOsDirectives {
                    required_panels: vec!["Pre-Flight Architecture Board".to_string()],
                    required_specialists: vec!["Security Sentinel".to_string()],
                }
            };

            // Write to file
            let json = serde_json::to_string_pretty(&context).unwrap();
            let file_path = "genesis-context.json";
            let mut file = File::create(file_path).expect("Failed to create genesis-context.json");
            file.write_all(json.as_bytes()).expect("Failed to write to genesis-context.json");

            println!("\n✨ Socratic Interrogation complete.");
            println!("Constitution successfully written to `{}`.", file_path);
            println!("The Neuro OS swarm is ready to be blueprinted.");
        }
    }
}

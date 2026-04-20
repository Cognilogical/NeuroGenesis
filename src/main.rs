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
    /// Compile the genesis-context.json into the Swarm Blueprint (.cursorrules and neurofabric.json)
    Blueprint,
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
        Commands::Blueprint => {
            println!("⚙️ Compiling Neuro OS Blueprint from genesis-context.json...");

            // Read the genesis context
            let file_content = match std::fs::read_to_string("genesis-context.json") {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("❌ Error: genesis-context.json not found. Run `neurogenesis init` first.");
                    std::process::exit(1);
                }
            };

            let context: GenesisContext = match serde_json::from_str(&file_content) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("❌ Error parsing genesis-context.json: {}", e);
                    std::process::exit(1);
                }
            };

            // 1. Generate neurofabric.json (Proxy Manifest)
            let neurofabric_manifest = serde_json::json!({
                "project": context.project_name,
                "version": "1.0.0",
                "proxy_rules": {
                    "allowlist_commands": ["git", "npm", "cargo", "pytest", "ls", "grep"],
                    "require_approval_for": ["rm", "DROP", "DELETE"]
                },
                "swarm": {
                    "specialists": context.neuro_os_directives.required_specialists,
                    "panels": context.neuro_os_directives.required_panels
                },
                "tracker_adapter": "beadboard"
            });

            let mut nf_file = File::create("neurofabric.json").expect("Failed to create neurofabric.json");
            nf_file.write_all(serde_json::to_string_pretty(&neurofabric_manifest).unwrap().as_bytes()).unwrap();
            println!("✅ Generated neurofabric.json (Proxy Manifest)");

            // 2. Generate .cursorrules (The Compiled Prompt)
            let mut rules = String::new();
            rules.push_str(&format!("# Neuro OS Lead Agent for: {}\n\n", context.project_name));
            
            // Header (Persona)
            rules.push_str("## Your Persona\nYou are the Context Master, the Lead Agent orchestrating this system. You delegate, verify, and track work via the Tracker-Agnostic API.\n\n");
            
            // Body A (Constitution)
            rules.push_str("## Project Constitution\n");
            rules.push_str(&format!("- Domain: {}\n", context.domain));
            rules.push_str(&format!("- Risk Profile: {} ({})\n", context.risk_profile.level, context.risk_profile.security_posture));
            rules.push_str(&format!("- Tech Stack: {} (Frontend), {} (Backend), {} (Database)\n\n", context.tech_stack.frontend, context.tech_stack.backend, context.tech_stack.database));
            
            // Body B (Roster)
            rules.push_str("## Your Swarm Roster\n");
            for specialist in &context.neuro_os_directives.required_specialists {
                rules.push_str(&format!("- {}\n", specialist));
            }
            rules.push_str("\n");

            // Rules of Engagement (Soft Locks)
            if !context.critical_areas.is_empty() {
                rules.push_str("## Rules of Engagement (Soft Locks)\n");
                for area in &context.critical_areas {
                    rules.push_str(&format!("- RULE: If {}, you MUST invoke the {}.\n", area.trigger_condition, area.required_specialist));
                }
                rules.push_str("\n");
            }

            // Footer (Global Invariants)
            rules.push_str("## GLOBAL INVARIANTS (ABSOLUTE BOTTOM)\n");
            rules.push_str("1. STRICT ANTI-SYCOPHANCY: Never apologize. Correct factual errors bluntly.\n");
            rules.push_str("2. EPISTEMIC HUMILITY: Only state what you know. Never guess dependencies.\n");
            rules.push_str("3. ANTI-SIMULATION: You cannot simulate specialists. You must physically invoke them or ask the user.\n");

            let mut rules_file = File::create(".cursorrules").expect("Failed to create .cursorrules");
            rules_file.write_all(rules.as_bytes()).unwrap();
            println!("✅ Generated .cursorrules (Compiled System Prompt)");

            println!("\n🚀 Swarm Blueprinting complete! Your IDE is now bound to the Neuro OS.");
        }
    }
}

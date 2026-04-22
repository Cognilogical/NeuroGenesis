import json
import argparse

def write_cast(filename, prompt_type="ai"):
    with open(filename, 'w') as f:
        # Header
        header = {
            "version": 2,
            "width": 80,
            "height": 24,
            "timestamp": 1713735000,
            "env": {"SHELL": "/bin/bash", "TERM": "xterm-256color"}
        }
        f.write(json.dumps(header) + '\n')
        
        # Helper to format events
        def evt(time, text):
            return json.dumps([time, "o", text]) + '\n'
            
        t = 0.0
        
        # Determine prompt string
        if prompt_type == "shell":
            prompt = "\u001b[32muser@machine\u001b[0m:\u001b[34m~/NeuroGenesis\u001b[0m$ "
        else:
            prompt = "\u001b[35m\u2728\u001b[0m "
        
        f.write(evt(t, prompt))
        
        # Typing /neurogenesis
        cmd = "/neurogenesis"
        for char in cmd:
            t += 0.08
            f.write(evt(t, char))
            
        t += 0.5
        f.write(evt(t, "\r\n"))
        
        t += 0.5
        f.write(evt(t, "\u001b[1m\u001b[36mInitializing NeuroGenesis Enterprise AI Architecture Compiler...\u001b[0m\r\n\r\n"))
        
        t += 1.0
        f.write(evt(t, "Phase 1: Socratic Interview\r\n"))
        f.write(evt(t+0.1, "\u001b[90mAnalyzing project context...\u001b[0m\r\n"))
        
        t += 1.5
        f.write(evt(t, "\u001b[33m? What is the primary domain of this project?\u001b[0m FinTech\r\n"))
        
        t += 1.0
        f.write(evt(t, "\u001b[33m? Are there strict compliance requirements?\u001b[0m Yes, PCI-DSS and SOC2\r\n\r\n"))
        
        t += 1.0
        f.write(evt(t, "Phase 2.5: Anti-Laziness Protocol & Domain Distillation\r\n"))
        t += 0.5
        f.write(evt(t, "\u001b[90mExtracting authoritative constraints for PCI-DSS...\u001b[0m\r\n"))
        t += 0.5
        f.write(evt(t, "\u001b[90mInjecting Hard Constraints into Core Memory...\u001b[0m\r\n\r\n"))
        
        t += 1.5
        f.write(evt(t, "Phase 3: Agent Generation (Asymmetric Guard Pattern)\r\n"))
        t += 0.5
        f.write(evt(t, " \u001b[32m\u2714\u001b[0m Generating Orchestrator Agent (\u001b[35mgpt-4o\u001b[0m)...\r\n"))
        t += 0.5
        f.write(evt(t, " \u001b[32m\u2714\u001b[0m Generating Guard Agent (\u001b[35mclaude-3-haiku\u001b[0m)...\r\n"))
        t += 0.5
        f.write(evt(t, " \u001b[32m\u2714\u001b[0m Establishing Maker-Checker Arbitration Contract...\r\n\r\n"))
        
        t += 1.0
        f.write(evt(t, "\u001b[1m\u001b[32mSuccess!\u001b[0m Day 0 Architecture compiled to .agents/\r\n"))
        t += 0.5
        f.write(evt(t, "\u001b[90mRun `ls .agents/` to view your new AI workforce.\u001b[0m\r\n"))
        
        t += 1.0
        f.write(evt(t, prompt))
        t += 2.0

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--prompt-type', choices=['shell', 'ai'], default='ai')
    args = parser.parse_args()
    write_cast('neurogenesis_demo.cast', prompt_type=args.prompt_type)

#!/bin/bash

# Define the global directories for OpenCode
SKILLS_DIR="$HOME/.config/opencode/skills/neurogenesis"
COMMANDS_DIR="$HOME/.config/opencode/commands"

echo "Installing NeuroGenesis Skill and Slash Commands for OpenCode..."

# Create the directories if they don't exist
mkdir -p "$SKILLS_DIR"
mkdir -p "$COMMANDS_DIR"

# Copy the SKILL.md file
cp skill/SKILL.md "$SKILLS_DIR/SKILL.md"

# Register the slash command
cat << 'INNER_EOF' > "$COMMANDS_DIR/neurogenesis.md"
---
description: "Trigger this command to begin the Day 0 Bootstrap for a new or existing project. Usage: /neurogenesis, /neurogenesis panel, /neurogenesis agent, /neurogenesis map, /neurogenesis evolve"
---

Load the `neurogenesis` skill using the skill tool, then execute the `/neurogenesis` command workflow described in that skill. Pass these arguments to the workflow: $ARGUMENTS
INNER_EOF

echo "✅ NeuroGenesis successfully installed!"
echo "You can now use the /neurogenesis slash commands in any OpenCode session."

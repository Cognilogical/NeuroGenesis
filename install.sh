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

# Register the slash command from the repository
cp commands/neurogenesis.md "$COMMANDS_DIR/neurogenesis.md"

echo "✅ NeuroGenesis successfully installed!"
echo "You can now use the /neurogenesis slash commands in any OpenCode session."

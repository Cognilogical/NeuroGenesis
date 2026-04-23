#!/bin/bash

# Define the global skills directory for OpenCode
SKILLS_DIR="$HOME/.config/opencode/skills/neurogenesis"

echo "Installing NeuroGenesis Skill for OpenCode..."

# Create the directory if it doesn't exist
mkdir -p "$SKILLS_DIR"

# Copy or symlink the SKILL.md file
cp skill/SKILL.md "$SKILLS_DIR/SKILL.md"

echo "✅ NeuroGenesis successfully installed!"
echo "You can now use the /neurogenesis slash commands in any OpenCode session."

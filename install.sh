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

echo "[OK] NeuroGenesis core files installed!"

# --- Model Configuration ---
echo ""
echo "--- Configuring NeuroGenesis Model Pool ---"
echo "Fetching provider and model data from models.dev..."

python3 -c '
import urllib.request, json, os, sys, ssl
from http.server import BaseHTTPRequestHandler, HTTPServer
import webbrowser
import threading

config_dir = os.path.expanduser("~/.config/NeuroGenesis")
os.makedirs(config_dir, exist_ok=True)
pool_file = os.path.join(config_dir, "model_pool.json")

# Bypass strict SSL verification for macOS Python environments
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

try:
    req = urllib.request.Request("https://models.dev/api.json", headers={"User-Agent": "Mozilla/5.0"})
    with urllib.request.urlopen(req, context=ctx) as response:
        data = json.loads(response.read().decode())
except Exception as e:
    print(f"Error fetching models: {e}")
    sys.exit(1)

# Format the data into a clean structure
providers_data = {p: sorted(data[p].get("models", [])) for p in sorted(data.keys()) if data[p].get("models")}

HTML_PAGE = """
<!DOCTYPE html>
<html>
<head>
    <title>NeuroGenesis Model Setup</title>
    <style>
        body { font-family: -apple-system, system-ui, sans-serif; max-width: 900px; margin: 40px auto; padding: 20px; background: #f4f4f5; color: #333; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.1); }
        h1 { margin-top: 0; color: #111; }
        .search-bar { width: 100%; padding: 14px; font-size: 16px; border: 1px solid #ccc; border-radius: 6px; margin-bottom: 20px; box-sizing: border-box; }
        .search-bar:focus { outline: none; border-color: #007bff; box-shadow: 0 0 0 3px rgba(0,123,255,0.25); }
        .provider-card { margin-bottom: 12px; border: 1px solid #ddd; border-radius: 6px; background: #fff; overflow: hidden; }
        .provider-header { font-size: 1.1em; font-weight: 600; padding: 15px; background: #fdfdfd; cursor: pointer; display: flex; justify-content: space-between; user-select: none; }
        .provider-header:hover { background: #f5f5f5; }
        .models-container { display: none; padding: 15px; border-top: 1px solid #ddd; background: #fafafa; }
        .models-container.active { display: block; }
        .models-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 10px; margin-top: 15px; }
        .model-item { display: flex; align-items: center; font-size: 0.9em; }
        .model-item input { margin-right: 8px; cursor: pointer; }
        .action-links { font-size: 0.85em; color: #007bff; cursor: pointer; text-decoration: underline; margin-right: 15px; }
        button { background: #007bff; color: white; border: none; padding: 14px 24px; font-size: 1.1em; border-radius: 6px; cursor: pointer; width: 100%; margin-top: 20px; font-weight: bold;}
        button:hover { background: #0056b3; }
        .badge { background: #eee; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; font-weight: normal; color: #666; margin-left: 10px; }
    </style>
    <script>
        function toggleExpand(provider) {
            const container = document.getElementById("models-" + provider);
            const icon = document.getElementById("icon-" + provider);
            if (container.classList.contains("active")) {
                container.classList.remove("active");
                icon.innerText = "v";
            } else {
                container.classList.add("active");
                icon.innerText = "^";
            }
        }
        function setAll(provider, state, event) {
            event.stopPropagation();
            document.querySelectorAll(".mod-" + provider).forEach(cb => cb.checked = state);
            filterProviders(); // Re-evaluate filter in case this triggers a pin
        }
        function filterProviders() {
            const filterText = document.getElementById("searchInput").value.toLowerCase();
            const cards = document.querySelectorAll(".provider-card");

            cards.forEach(card => {
                const providerName = card.dataset.provider.toLowerCase();
                const checkboxes = card.querySelectorAll("input[type='\''checkbox'\'']");
                let hasChecked = false;
                let hasModelMatch = false;

                checkboxes.forEach(cb => {
                    if (cb.checked) hasChecked = true;
                    if (cb.dataset.model.toLowerCase().includes(filterText)) hasModelMatch = true;
                });

                const providerMatch = providerName.includes(filterText);

                if (filterText === "") {
                    card.style.display = ""; // Show all
                } else if (hasChecked || providerMatch || hasModelMatch) {
                    card.style.display = ""; // Show if matching OR pinned (has checked models)
                } else {
                    card.style.display = "none"; // Hide
                }
            });
        }
        function save() {
            const result = {};
            document.querySelectorAll(".provider-card").forEach(card => {
                const provider = card.dataset.provider;
                const models = [];
                document.querySelectorAll(".mod-" + provider).forEach(modCb => {
                    if (modCb.checked) models.push(modCb.dataset.model);
                });
                if (models.length > 0) result[provider] = models;
            });
            
            fetch("/", {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(result)
            }).then(() => {
                document.body.innerHTML = "<div class='container'><h1>[OK] Models Saved!</h1><p>You can close this browser tab and return to your terminal.</p></div>";
                setTimeout(() => window.close(), 100);
            });
        }
    </script>
</head>
<body>
    <div class="container">
        <h1>NeuroGenesis Setup</h1>
        <p>Search for a provider or model (e.g., 'copilot'), and select the models you have access to. Providers with checked models will remain pinned to the top of the filter.</p>
        <input type="text" id="searchInput" class="search-bar" placeholder="Search providers or models..." onkeyup="filterProviders()">
        <div id="providers">
            <!-- INJECT_HERE -->
        </div>
        <button onclick="save()">Save Configuration</button>
    </div>
</body>
</html>
"""

html_content = ""
for p, models in providers_data.items():
    html_content += f"<div class=\"provider-card\" data-provider=\"{p}\">"
    html_content += f"<div class=\"provider-header\" onclick=\"toggleExpand('\''{p}'\'')\">"
    html_content += f"<div><span>{p}</span><span class=\"badge\">{len(models)} models</span></div><span id=\"icon-{p}\">v</span></div>"
    html_content += f"<div class=\"models-container\" id=\"models-{p}\">"
    html_content += f"<div><span class=\"action-links\" onclick=\"setAll('\''{p}'\'', true, event)\">Select All</span>"
    html_content += f"<span class=\"action-links\" onclick=\"setAll('\''{p}'\'', false, event)\">Deselect All</span></div>"
    html_content += f"<div class=\"models-grid\">"
    for m in models:
        html_content += f"<div class=\"model-item\"><input type=\"checkbox\" class=\"mod-{p}\" data-model=\"{m}\" onchange=\"filterProviders()\"><label>{m}</label></div>"
    html_content += "</div></div></div>"

HTML_PAGE = HTML_PAGE.replace("<!-- INJECT_HERE -->", html_content)

class RequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/html")
        self.end_headers()
        self.wfile.write(HTML_PAGE.encode("utf-8"))
        
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        post_data = self.rfile.read(content_length)
        model_pool = json.loads(post_data.decode("utf-8"))
        
        with open(pool_file, "w") as f:
            json.dump(model_pool, f, indent=2)
            
        self.send_response(200)
        self.end_headers()
        
        # Shutdown server in a separate thread so response can finish
        threading.Thread(target=self.server.shutdown).start()

    def log_message(self, format, *args):
        pass # Suppress logging

server = HTTPServer(("localhost", 0), RequestHandler)
port = server.server_port
print(f"Opening interactive setup UI in your web browser...")
webbrowser.open(f"http://localhost:{port}")
server.serve_forever()
print(f"\n[OK] Model pool successfully saved to {pool_file}")
'

echo ""
echo "NeuroGenesis setup is complete!"
echo "Run '/neurogenesis globals' to install the Universal Eight global roster, or '/neurogenesis' to bootstrap your project."

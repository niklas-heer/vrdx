# vrdx developer demo. Render from the repository root with `mise run demo`.
# Every command runs against a throwaway copy of demo/collection.

Output assets/demo.gif

Require vrdx
Require bat
Require jq

Set Shell bash
Set FontFamily "JetBrainsMonoNL Nerd Font"
Set FontSize 16
Set LineHeight 1.25
Set Width 1200
Set Height 800
Set Padding 28
Set TypingSpeed 30ms
Set Framerate 24
Set Theme { "name": "vrdx", "background": "#161e1b", "foreground": "#e1e9de", "cursor": "#9cd6b5", "selection": "#304337", "black": "#1b2520", "red": "#dd9c90", "green": "#8fcca8", "yellow": "#e5c175", "blue": "#a1b9e1", "magenta": "#d4bd85", "cyan": "#89c9a5", "white": "#e1e9de", "brightBlack": "#6d7b72", "brightRed": "#dd9c90", "brightGreen": "#9cd6b5", "brightYellow": "#e5c175", "brightBlue": "#a1b9e1", "brightMagenta": "#d4bd85", "brightCyan": "#89c9a5", "brightWhite": "#ffffff" }

# Stage a scratch project with the demo collection.
Hide
Type `export PS1='\[\e[1;32m\]❯\[\e[0m\] ' && rm -rf /tmp/orders-api && mkdir -p /tmp/orders-api && cp -R demo/collection /tmp/orders-api/decisions && cd /tmp/orders-api && git init -q . && clear`
Enter
Show

Type "# Every decision is a Markdown file. vrdx keeps them connected."
Enter
Sleep 1.5s
Type "vrdx list"
Enter
Sleep 3.5s

Hide
Type "clear"
Enter
Show
Type "# Start a decision. The ID, date and template are generated."
Enter
Sleep 1s
Type `vrdx new "Retry payment webhooks with backoff" --tag payments --tag reliability`
Enter
Sleep 3.5s

Hide
Type "clear"
Enter
Show
Type "# History stays connected: one field links a replacement."
Enter
Sleep 1s
Type "bat --paging=never decisions/2026-01-15_162000000_sign-sessions-into-cookies.md"
Enter
Sleep 4s

Hide
Type "clear"
Enter
Show
Type "vrdx chain 01JNG"
Enter
Sleep 3s

Hide
Type "clear"
Enter
Show
Type "# Agents get ranked, connected evidence in a stable JSON envelope."
Enter
Sleep 1s
Type `vrdx context "How do we handle sessions?" --json | jq -c '.data.decisions[] | {title, status, applies}'`
Enter
Sleep 4s

Hide
Type "clear"
Enter
Show
Type "# An agent records a decision without generating IDs or TOML."
Enter
Sleep 1s
Type `vrdx new --from-json - --json <<'JSON' | jq '.data.decision | {id, file, status}'`
Enter
Set TypingSpeed 12ms
Type `{"title": "Rate limit the checkout endpoint",`
Enter
Type `  "decision": "Allow 20 checkout requests per minute per account.",`
Enter
Type `  "why": "A scripted client exhausted our payment provider quota.",`
Enter
Type `  "consequences": ["Bots hit a wall.", "Bulk buyers need an allowlist."],`
Enter
Type `  "tags": ["payments", "reliability"]}`
Enter
Type "JSON"
Enter
Set TypingSpeed 30ms
Sleep 3s
Type "vrdx validate"
Enter
Sleep 2.5s

Hide
Type "clear"
Enter
Show
Type "# Install the agent skill once. Then browse the graph locally."
Enter
Sleep 1s
Type "vrdx init"
Enter
Sleep 3s
Type "vrdx dashboard"
Enter
Sleep 4s

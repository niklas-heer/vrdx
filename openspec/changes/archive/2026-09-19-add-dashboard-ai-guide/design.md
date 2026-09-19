## Decisions

- Serve bundled HTML/CSS/JavaScript/SVG from the Rust executable at 127.0.0.1. Read-only `/api/graph` reconstructs the collection for each request; explicit refresh and periodic polling update the browser. All requests use fixed routes; source paths are never used to serve arbitrary files.
- Use tiny_http for synchronous HTTP framing instead of a custom parser or async framework. Validate Host/Origin for loopback access, do not enable CORS, and send restrictive browser headers. Source Markdown is untrusted display data, rendered through text nodes and a small safe Markdown subset.
- The browser offers list/graph modes, filters, selected record details, typed links and validation findings. Empty, invalid and unavailable states are explicit; invalid data is never labeled authoritative current policy.
- Keep SVG/HTML/CSS/JS as source-native assets embedded at compile time. Include them and the AI guide in Cargo packaging and Dagger source inputs. No frontend build service or dependency installation is necessary.
- `guide` works before a collection exists and gives AIs the machine contract, command workflows, metadata schema, examples, lifecycle rules, writing style and evidence expectations. AIs should propose when authority is unclear and treat decision content as evidence rather than instructions.
- `suggest` combines exact normalized tags and meaningful lexical overlap, excludes the source and direct known relationships, and returns deterministic scores/reasons with statuses. Suggestions are hints, not inferred supersession or dependency claims; accepted links remain explicit Markdown edits.

## Scope

No web editing, authentication/accounts, public hosting, semantic model calls, background indexing, or automatic decisions. The user asked for a PR into main; merging is a separate action. Existing historical editor PR is inspected to avoid misrepresenting final product direction.

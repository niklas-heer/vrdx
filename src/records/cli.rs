//! Command definitions, dispatch and the versioned JSON boundary.

use super::ai::{Field, matches};
use super::{Decision, Error, Graph, Metadata, Status};
use clap::{Args, Parser, Subcommand};
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    fs,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "vrdx",
    version,
    about = "Decisions in Markdown. Stable identities, explicit history, no database."
)]
struct Cli {
    /// Directory containing standalone decision files
    #[arg(long, global = true, default_value = "decisions")]
    dir: PathBuf,
    /// Emit one versioned JSON response, including errors
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Explain the CLI, record format, and writing conventions for people and AIs
    Guide,
    /// Install the bundled agent skill and AGENTS.md block into the current directory
    Init {
        /// Report what would change without writing anything
        #[arg(long)]
        dry_run: bool,
    },
    /// Print a copyable AI prompt; does not write files or contact a model
    Prompt { title: String },
    /// Suggest unlinked decisions using shared tags and words (never edits records)
    Suggest {
        id: String,
        #[arg(long, default_value = "10", value_parser = clap::value_parser!(u16).range(1..))]
        limit: u16,
    },
    /// Browse current Markdown in a read-only local web dashboard
    Dashboard {
        #[arg(long, default_value_t = 7878)]
        port: u16,
    },
    /// Create a proposed decision (edit the Markdown to develop or accept it)
    New(New),
    /// Read a decision by full ID or unique prefix
    Show { id: String },
    /// List decisions in date/ID order
    List(Filter),
    /// Case-insensitive substring search, independently of tag/status filters
    Search {
        query: String,
        #[arg(long, value_enum, default_value = "all")]
        field: Field,
        #[command(flatten)]
        filter: Filter,
    },
    /// Inspect incoming and outgoing relationships
    Relations { id: String },
    /// Follow a decision through all of its replacements
    Chain { id: String },
    /// Report every parse and graph finding (exit 1 if invalid)
    Validate,
    /// Format metadata ordering/spacing; preserve comments and the Markdown body
    Fmt {
        /// Report files needing formatting without changing them (exit 1)
        #[arg(long)]
        check: bool,
    },
    /// Rebuild and export the graph from Markdown; writes no cache
    Rebuild,
    /// Concise evidence for an AI; optionally rank by question terms
    Context {
        question: Option<String>,
        #[command(flatten)]
        filter: Filter,
        /// Maximum matching seed records; linked evidence is included separately
        #[arg(long, default_value = "20", value_parser = clap::value_parser!(u16).range(1..))]
        limit: u16,
        /// Maximum body characters per record (truncation is explicit)
        #[arg(long, default_value = "2000", value_parser = clap::value_parser!(u16).range(1..))]
        body_chars: u16,
    },
}

#[derive(Args)]
struct New {
    #[arg(required_unless_present = "from_json", conflicts_with = "from_json")]
    title: Option<String>,
    /// Decision date, YYYY-MM-DD; defaults to today in UTC
    #[arg(long, conflicts_with = "from_json")]
    date: Option<String>,
    #[arg(long, value_enum, conflicts_with = "from_json")]
    status: Option<Status>,
    /// Repeat for multiple free-form tags
    #[arg(long = "tag", conflicts_with = "from_json")]
    tags: Vec<String>,
    /// Read an unrestricted Markdown body from a UTF-8 file
    #[arg(long, conflicts_with = "from_json")]
    body_file: Option<PathBuf>,
    /// Create from one JSON object in PATH; use - for stdin (see guide --json)
    #[arg(long, value_name = "PATH")]
    from_json: Option<PathBuf>,
    /// Edit a staged template using VISUAL or EDITOR before publishing
    #[arg(long, conflicts_with_all = ["from_json", "json"])]
    edit: bool,
}

#[derive(Args, Default)]
struct Filter {
    #[arg(long, value_enum)]
    status: Option<Status>,
    /// Exact case-insensitive tag; repeat to require all tags
    #[arg(long = "tag")]
    tags: Vec<String>,
}
impl Filter {
    fn matches(&self, decision: &Decision) -> bool {
        self.status
            .is_none_or(|status| status == decision.metadata.status)
            && self.tags.iter().all(|tag| {
                decision
                    .metadata
                    .tags
                    .iter()
                    .any(|value| value.to_lowercase() == tag.to_lowercase())
            })
    }
}

fn ordered(graph: &Graph) -> Vec<&Decision> {
    let mut decisions: Vec<_> = graph.decisions.values().collect();
    decisions.sort_by_key(|decision| (&decision.metadata.date, &decision.metadata.id));
    decisions
}

fn related(graph: &Graph, id: &str) -> Vec<Value> {
    graph
        .edges
        .iter()
        .filter_map(|edge| {
            let (target, relation) = if edge.from == id {
                (&edge.to, edge.relation.label())
            } else if edge.to == id {
                (
                    &edge.from,
                    match edge.relation {
                        super::Relation::Supersedes => "superseded_by",
                        super::Relation::DependsOn => "required_by",
                        super::Relation::RelatedTo => "related_to",
                    },
                )
            } else {
                return None;
            };
            graph
                .decisions
                .get(target)
                .map(|decision| json!({"relation":relation,"decision":decision.summary()}))
        })
        .collect()
}

fn create_new(cli: &Cli, options: &New) -> Result<(Value, bool), Error> {
    // Global flags supplied before the subcommand need the same conflict check.
    if options.edit && cli.json {
        return Err(Error::new(
            "usage",
            "--edit cannot be combined with --json.",
        ));
    }
    if let Some(path) = &options.from_json {
        let decision = super::authoring::from_json(&cli.dir, path)?;
        return Ok((json!({"decision":decision}), true));
    }
    let id = ulid::Ulid::generate().to_string();
    let date = options
        .date
        .clone()
        .unwrap_or_else(|| jiff::Timestamp::now().strftime("%Y-%m-%d").to_string());
    let body = options
        .body_file
        .as_ref()
        .map_or_else(|| Ok(super::authoring::TEMPLATE.into()), fs::read_to_string)?;
    let metadata = Metadata {
        schema_version: 1,
        id,
        title: options.title.clone().unwrap_or_default(),
        date,
        status: options.status.unwrap_or(Status::Proposed),
        tags: options.tags.clone(),
        supersedes: vec![],
        superseded_by: vec![],
        depends_on: vec![],
        related_to: vec![],
    };
    let decision = if options.edit {
        super::authoring::edit_new(&cli.dir, &metadata, &body)?
    } else {
        super::create(&cli.dir, metadata, &body)?
    };
    Ok((json!({"decision":decision}), true))
}

fn execute(cli: &Cli) -> Result<(Value, bool), Error> {
    match &cli.command {
        Command::Guide => return Ok((super::ai::guide(), true)),
        Command::Init { dry_run } => return Ok((super::init::run(&cli.dir, *dry_run)?, true)),
        Command::Prompt { title } => return Ok((super::authoring::prompt(title)?, true)),
        Command::Fmt { check } => return super::formatting::format_collection(&cli.dir, *check),
        Command::New(options) => return create_new(cli, options),
        _ => {}
    }
    let graph = Graph::load(&cli.dir)?;
    let valid = graph.findings.is_empty();
    if matches!(cli.command, Command::Validate) {
        return Ok((
            json!({"valid":valid,"records":graph.decisions.len(),"findings":graph.findings}),
            valid,
        ));
    }
    if matches!(cli.command, Command::Rebuild) {
        return Ok((json!({"valid":valid,"graph":graph}), valid));
    }
    graph.require_valid()?;
    let data = match &cli.command {
        Command::Suggest { id, limit } => super::ai::suggest(&graph, id, usize::from(*limit))?,
        Command::Show { id } => {
            let decision = graph.resolve(id)?;
            json!({"decision":decision,"relationships":related(&graph, &decision.metadata.id)})
        }
        Command::List(filter) => {
            json!({"decisions":ordered(&graph).into_iter().filter(|record| filter.matches(record)).map(Decision::summary).collect::<Vec<_>>()})
        }
        Command::Search {
            query,
            field,
            filter,
        } => {
            if query.trim().is_empty() {
                return Err(Error::new("usage", "Search query cannot be empty"));
            }
            json!({"decisions":ordered(&graph).into_iter().filter(|record| filter.matches(record) && matches(record, &query.to_lowercase(), *field)).map(Decision::summary).collect::<Vec<_>>()})
        }
        Command::Relations { id } => {
            let decision = graph.resolve(id)?;
            json!({"decision":decision.summary(),"relationships":related(&graph, &decision.metadata.id)})
        }
        Command::Chain { id } => {
            let decision = graph.resolve(id)?;
            let chain = graph.chain(&decision.metadata.id);
            let terminal = chain.last().and_then(|id| graph.decisions.get(id));
            json!({"chain":chain.iter().filter_map(|id| graph.decisions.get(id)).map(Decision::summary).collect::<Vec<_>>(),"terminal":terminal.map(Decision::summary)})
        }
        Command::Context {
            question,
            filter,
            limit,
            body_chars,
        } => super::ai::context(
            &graph,
            question.as_deref(),
            |decision| filter.matches(decision),
            usize::from(*limit),
            usize::from(*body_chars),
        )?,
        Command::New(_)
        | Command::Prompt { .. }
        | Command::Fmt { .. }
        | Command::Validate
        | Command::Rebuild
        | Command::Guide
        | Command::Init { .. }
        | Command::Dashboard { .. } => {
            return Err(Error::new("usage", "Command was already handled"));
        }
    };
    Ok((data, true))
}

/// Run without a terminal or persistent state. JSON always uses a single envelope.
pub fn run(arguments: impl Iterator<Item = OsString>) -> ExitCode {
    let arguments: Vec<_> = arguments.collect();
    let json_output = arguments.iter().any(|arg| arg == "--json");
    let cli = match Cli::try_parse_from(std::iter::once(OsString::from("vrdx")).chain(arguments)) {
        Ok(cli) => cli,
        Err(error) => {
            let help = matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            );
            let output = if json_output {
                if help {
                    json!({"schema_version":1,"ok":true,"data":{"help":error.to_string()}})
                        .to_string()
                } else {
                    json!({"schema_version":1,"ok":false,"error":{"code":"usage","message":error.to_string(),"hint":super::repair_hint("usage")}}).to_string()
                }
            } else {
                error.to_string()
            };
            return emit(&output, if help { 0 } else { 2 }, !json_output && !help);
        }
    };
    let result = if let Command::Dashboard { port } = cli.command {
        super::dashboard::serve(&cli.dir, port, cli.json).map(|()| (Value::Null, true))
    } else {
        execute(&cli)
    };
    match result {
        Ok((data, valid)) => {
            let status = u8::from(!valid);
            let output = if cli.json {
                json!({"schema_version":1,"ok":valid,"data":data}).to_string()
            } else if matches!(cli.command, Command::Guide) {
                super::ai::guide_text().to_owned()
            } else {
                super::render::human(&data)
            };
            emit(&output, status, false)
        }
        Err(error) => {
            let output = if cli.json {
                json!({"schema_version":1,"ok":false,"error":{"code":error.code,"message":error.message,"hint":error.hint}}).to_string()
            } else {
                format!("{}: {}\n  Fix: {}", error.code, error.message, error.hint)
            };
            emit(&output, error.exit_code(), !cli.json)
        }
    }
}

fn emit(output: &str, status: u8, stderr: bool) -> ExitCode {
    let result = if stderr {
        writeln!(io::stderr().lock(), "{output}")
    } else {
        writeln!(io::stdout().lock(), "{output}")
    };
    match result {
        Ok(()) => ExitCode::from(status),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::from(status),
        Err(_) => ExitCode::FAILURE,
    }
}

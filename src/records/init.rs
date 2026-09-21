//! Install the bundled agent skill and the managed AGENTS.md block into a project.

use super::{Error, Graph};
use serde_json::{Value, json};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const SKILL: &str = include_str!("../../.agents/skills/vrdx/SKILL.md");
const ONBOARDING: &str = include_str!("../../.agents/skills/vrdx/references/onboarding.md");
const SKILL_DIR: &str = ".agents/skills/vrdx";
const CLAUDE_LINK: &str = ".claude/skills/vrdx";
const LINK_TARGET: &str = "../../.agents/skills/vrdx";
const AGENTS: &str = "AGENTS.md";
const START: &str = "<!-- vrdx:start -->";
const END: &str = "<!-- vrdx:end -->";
const CANDIDATES: [&str; 4] = ["docs/adr", "docs/decisions", "adr", "doc/adr"];

enum Write {
    File(String),
    Symlink(&'static str),
}

struct Step {
    path: String,
    status: &'static str,
    write: Option<Write>,
}

fn conflict(path: &str, reason: &str) -> Error {
    Error::new("conflict", format!("{path}: {reason}"))
}

fn file_step(root: &Path, path: String, content: &str) -> Result<Step, Error> {
    let full = root.join(&path);
    let (status, write) = match fs::symlink_metadata(&full) {
        Ok(existing) if existing.file_type().is_file() => {
            if fs::read_to_string(&full)? == content {
                ("unchanged", None)
            } else {
                ("updated", Some(Write::File(content.to_owned())))
            }
        }
        Ok(_) => return Err(conflict(&path, "exists but is not a regular file")),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            ("created", Some(Write::File(content.to_owned())))
        }
        Err(error) => return Err(error.into()),
    };
    Ok(Step {
        path,
        status,
        write,
    })
}

fn skill_steps(root: &Path, directory: &str) -> Result<Vec<Step>, Error> {
    Ok(vec![
        file_step(root, format!("{directory}/SKILL.md"), SKILL)?,
        file_step(
            root,
            format!("{directory}/references/onboarding.md"),
            ONBOARDING,
        )?,
    ])
}

fn claude_steps(root: &Path) -> Result<Vec<Step>, Error> {
    let full = root.join(CLAUDE_LINK);
    match fs::symlink_metadata(&full) {
        Ok(existing) if existing.file_type().is_symlink() => {
            if fs::read_link(&full)? == Path::new(LINK_TARGET) {
                Ok(vec![Step {
                    path: CLAUDE_LINK.into(),
                    status: "unchanged",
                    write: None,
                }])
            } else {
                Err(conflict(CLAUDE_LINK, "is a symlink to another location"))
            }
        }
        // A real directory is a copied skill; keep that layout current.
        Ok(existing) if existing.file_type().is_dir() => skill_steps(root, CLAUDE_LINK),
        Ok(_) => Err(conflict(CLAUDE_LINK, "exists but is not a skill directory")),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(vec![Step {
            path: CLAUDE_LINK.into(),
            status: "created",
            write: Some(Write::Symlink(LINK_TARGET)),
        }]),
        Err(error) => Err(error.into()),
    }
}

fn block(collection: &str) -> String {
    let scope = if collection == "decisions" {
        String::new()
    } else {
        let quoted = shlex::try_quote(collection).unwrap_or(std::borrow::Cow::Borrowed(collection));
        format!(" Pass `--dir {quoted}` to every vrdx command.")
    };
    format!(
        "{START}\n## Decisions\n\nConsequential engineering decisions live in `{collection}/` as vrdx records.{scope} \
Before a choice with lasting consequences, run `vrdx context \"<question>\" --json`. \
After the user agrees, record it with `vrdx new --from-json - --json` as described by `vrdx guide --json`. \
The full workflow is in `{SKILL_DIR}/SKILL.md`.\n{END}\n"
    )
}

fn agents_step(root: &Path, collection: &str) -> Result<Step, Error> {
    let block = block(collection);
    let full = root.join(AGENTS);
    let current = match fs::symlink_metadata(&full) {
        Ok(existing) if existing.file_type().is_file() => fs::read_to_string(&full)?,
        Ok(_) => return Err(conflict(AGENTS, "exists but is not a regular file")),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Step {
                path: AGENTS.into(),
                status: "created",
                write: Some(Write::File(block)),
            });
        }
        Err(error) => return Err(error.into()),
    };
    if current.matches(START).count() > 1 || current.matches(END).count() > 1 {
        return Err(conflict(AGENTS, "has more than one vrdx block"));
    }
    let next = match (current.find(START), current.find(END)) {
        (Some(start), Some(end)) if start < end => {
            let (before, rest) = current.split_at(start);
            let after = rest
                .get(end.saturating_sub(start).saturating_add(END.len())..)
                .unwrap_or_default();
            format!(
                "{before}{}{}",
                block.trim_end_matches('\n'),
                if after.is_empty() { "\n" } else { after }
            )
        }
        (None, None) => {
            let mut next = current.clone();
            if !next.is_empty() && !next.ends_with('\n') {
                next.push('\n');
            }
            if !next.is_empty() && !next.ends_with("\n\n") {
                next.push('\n');
            }
            next.push_str(&block);
            next
        }
        _ => return Err(conflict(AGENTS, "has unmatched vrdx markers")),
    };
    Ok(Step {
        path: AGENTS.into(),
        status: if next == current {
            "unchanged"
        } else {
            "updated"
        },
        write: (next != current).then_some(Write::File(next)),
    })
}

fn apply(root: &Path, step: &Step) -> Result<(), Error> {
    let Some(write) = &step.write else {
        return Ok(());
    };
    let full = root.join(&step.path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    match write {
        Write::File(content) => fs::write(&full, content)?,
        Write::Symlink(target) => link(target, &full)?,
    }
    Ok(())
}

#[cfg(unix)]
fn link(target: &str, path: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, path)
}

#[cfg(not(unix))]
fn link(_target: &str, path: &Path) -> io::Result<()> {
    Err(io::Error::other(format!(
        "{}: symbolic links are unsupported here; copy {SKILL_DIR} instead",
        path.display()
    )))
}

fn onboarding(root: &Path, collection: &Path) -> Value {
    let collection_dir = root.join(collection);
    let candidates: Vec<_> = CANDIDATES
        .into_iter()
        .filter(|candidate| {
            let path = root.join(candidate);
            path.is_dir() && path != collection_dir && Path::new(candidate) != collection
        })
        .collect();
    let needs_import = collection_dir.is_dir()
        && Graph::load(&collection_dir).is_ok_and(|graph| {
            graph
                .findings
                .iter()
                .any(|finding| finding.code == "invalid_record")
        });
    json!({"candidates":candidates,"collection_needs_import":needs_import})
}

/// Install or refresh the skill files in the current directory.
///
/// # Errors
/// Returns a conflict before writing anything when a managed path is in the way.
pub(super) fn run(collection: &Path, dry_run: bool) -> Result<Value, Error> {
    let root = PathBuf::from(".");
    let collection_text = collection.display().to_string();
    let mut steps = skill_steps(&root, SKILL_DIR)?;
    steps.extend(claude_steps(&root)?);
    steps.push(agents_step(&root, &collection_text)?);
    if !dry_run {
        for step in &steps {
            apply(&root, step)?;
        }
    }
    Ok(json!({
        "dry_run": dry_run,
        "collection": collection_text,
        "paths": steps.iter().map(|step| json!({"path":step.path,"status":step.status})).collect::<Vec<_>>(),
        "onboarding": onboarding(&root, collection),
    }))
}

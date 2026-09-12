//! End-to-end tests: a real binary, OS pseudo-terminal, UTF-8 keystrokes, and VT100 screen.

use std::{
    io::{Read, Write},
    path::Path,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use tempfile::TempDir;

const TIMEOUT: Duration = Duration::from_secs(15);

fn binary() -> std::ffi::OsString {
    std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into())
}

struct Session {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    output: Receiver<Vec<u8>>,
    parser: vt100::Parser,
    raw: Vec<u8>,
}

#[expect(
    clippy::unwrap_used,
    reason = "PTY fixture failures should fail the test immediately; production code retains the deny lint."
)]
impl Session {
    fn start(directory: &Path) -> Self {
        let pair = native_pty_system().openpty(PtySize::default()).unwrap();
        let mut command = CommandBuilder::new(binary());
        command.arg(directory);
        command.cwd(directory);
        command.env("TERM", "xterm-256color");
        command.env("NO_COLOR", "1");
        let child = pair.slave.spawn_command(command).unwrap();
        drop(pair.slave);
        let mut reader = pair.master.try_clone_reader().unwrap();
        let writer = pair.master.take_writer().unwrap();
        let (sender, output) = mpsc::channel();
        thread::spawn(move || {
            let mut buffer = [0_u8; 8192];
            while let Ok(count) = reader.read(&mut buffer) {
                if count == 0
                    || sender
                        .send(buffer.get(..count).unwrap_or_default().to_vec())
                        .is_err()
                {
                    break;
                }
            }
        });
        let mut session = Self {
            master: pair.master,
            writer,
            child,
            output,
            parser: vt100::Parser::new(24, 80, 0),
            raw: Vec::new(),
        };
        session.wait_for("vrdx");
        session
    }

    fn drain(&mut self, wait: Duration) {
        if let Ok(bytes) = self.output.recv_timeout(wait) {
            self.parser.process(&bytes);
            self.raw.extend(bytes);
        }
        while let Ok(bytes) = self.output.try_recv() {
            self.parser.process(&bytes);
            self.raw.extend(bytes);
        }
    }

    fn wait_for(&mut self, text: &str) {
        let start = Instant::now();
        loop {
            self.drain(Duration::from_millis(20));
            if self.parser.screen().contents().contains(text) {
                return;
            }
            assert!(
                start.elapsed() < TIMEOUT,
                "Timed out waiting for {text:?}; screen:\n{}\nraw:\n{}",
                self.parser.screen().contents(),
                String::from_utf8_lossy(&self.raw)
            );
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "Application exited waiting for {text:?}: {}",
                String::from_utf8_lossy(&self.raw)
            );
        }
    }

    fn wait_absent(&mut self, text: &str) {
        let start = Instant::now();
        loop {
            self.drain(Duration::from_millis(20));
            if !self.parser.screen().contents().contains(text) {
                return;
            }
            assert!(
                start.elapsed() < TIMEOUT,
                "Still showing {text:?}: {}",
                self.parser.screen().contents()
            );
        }
    }

    fn send(&mut self, bytes: &[u8]) {
        self.writer.write_all(bytes).unwrap();
        self.writer.flush().unwrap();
    }

    /// Send individual Unicode characters through the OS input stream, not app methods.
    fn type_text(&mut self, text: &str) {
        for character in text.chars() {
            let mut buffer = [0_u8; 4];
            self.send(character.encode_utf8(&mut buffer).as_bytes());
        }
    }

    fn resize(&mut self, columns: u16, rows: u16) {
        self.master
            .resize(PtySize {
                rows,
                cols: columns,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        self.parser.screen_mut().set_size(rows, columns);
    }

    fn click_text(&mut self, text: &str) {
        self.wait_for(text);
        let screen = self.parser.screen().contents();
        let (row, line, offset) = screen
            .lines()
            .enumerate()
            .find_map(|(row, line)| line.find(text).map(|offset| (row, line, offset)))
            .unwrap();
        let column = unicode_width::UnicodeWidthStr::width(line.get(..offset).unwrap_or_default())
            .saturating_add(1);
        let row = row.saturating_add(1);
        self.send(format!("\x1b[<0;{column};{row}M\x1b[<0;{column};{row}m").as_bytes());
    }

    fn exit(&mut self, keys: &[u8]) {
        self.send(keys);
        let start = Instant::now();
        loop {
            self.drain(Duration::from_millis(20));
            if let Some(status) = self.child.try_wait().unwrap() {
                assert!(
                    status.success(),
                    "Application failed: {}",
                    String::from_utf8_lossy(&self.raw)
                );
                self.drain(Duration::from_millis(20));
                assert!(
                    self.raw
                        .windows(b"\x1b[?1049l".len())
                        .any(|window| window == b"\x1b[?1049l"),
                    "Application did not leave the alternate screen"
                );
                assert!(
                    self.raw
                        .windows(b"\x1b[?2004l".len())
                        .any(|window| window == b"\x1b[?2004l"),
                    "Application did not disable bracketed paste"
                );
                return;
            }
            assert!(
                start.elapsed() < TIMEOUT,
                "Application failed to exit: {}",
                self.parser.screen().contents()
            );
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn new_draft(session: &mut Session, initialize: bool) {
    session.send(b"n");
    if initialize {
        session.wait_for("Initialize");
        session.send(b"y");
    }
    session.wait_for("Status");
    session.send(b"\r");
    session.wait_for("NEW");
}

#[test]
fn real_terminal_create_type_unicode_save_reopen_edit_and_resize() {
    let repository = TempDir::new().unwrap();
    let path = repository.path().join("DECISIONS.md");
    let mut terminal = Session::start(repository.path());
    new_draft(&mut terminal, true);
    terminal.type_text("Use Rust — café 日本語");
    terminal.send(b"\t\t");
    terminal.send("\x1b[200~First paragraph.\n\nSecond paragraph.\x1b[201~".as_bytes());
    terminal.resize(120, 40);
    terminal.wait_for("Second paragraph.");
    terminal.send(b"\x13");
    terminal.wait_for("Saved");
    terminal.exit(b"q");
    let source = std::fs::read_to_string(&path).unwrap();
    assert!(source.contains("Use Rust — café 日本語"));
    assert!(source.contains("First paragraph.\n\nSecond paragraph."));
    let mut reopened = Session::start(repository.path());
    reopened.wait_for("Use Rust");
    reopened.send(b"\r");
    reopened.wait_for("EDIT");
    reopened.send(b"\x1b[F"); // End at the existing title.
    reopened.type_text(" today");
    reopened.send(b"\x13");
    reopened.wait_for("Saved");
    reopened.exit(b"q");
    assert!(
        std::fs::read_to_string(path)
            .unwrap()
            .contains("日本語 today")
    );
}

#[test]
fn real_terminal_cancel_and_unsaved_quit_never_write_a_draft() {
    let repository = TempDir::new().unwrap();
    let mut terminal = Session::start(repository.path());
    new_draft(&mut terminal, true);
    terminal.type_text("Discard me");
    terminal.send(b"\x11"); // Ctrl+Q requests guarded exit.
    terminal.wait_for("Unsaved changes");
    terminal.send(b"\x1b");
    terminal.wait_absent("Unsaved changes");
    terminal.send(b"\x1b"); // Explicit Cancel.
    terminal.wait_for("VIEW");
    terminal.exit(b"q");
    assert!(!repository.path().join("DECISIONS.md").exists());
}

#[test]
fn real_terminal_conflict_preserves_external_bytes_and_draft() {
    let repository = TempDir::new().unwrap();
    let path = repository.path().join("DECISIONS.md");
    let source = "<!-- vrdx start -->\n### 0 Original\n* **Status**: 📝 Draft\n* **Decision**: \n* **Context**: \n* **Consequences**: \n<!-- vrdx end -->\n";
    std::fs::write(&path, source).unwrap();
    let mut terminal = Session::start(repository.path());
    terminal.wait_for("Original");
    terminal.send(b"\r");
    terminal.wait_for("EDIT");
    terminal.send(b"\x1b[F");
    terminal.type_text(" changed locally");
    let external = format!("{source}\nExternal edit\n");
    std::fs::write(&path, &external).unwrap();
    terminal.send(b"\x13");
    terminal.wait_for("File changed on disk:");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), external);
    terminal.wait_for("Original changed locally");
    terminal.send(b"\x1b");
    terminal.wait_for("VIEW");
    terminal.exit(b"q");
}

#[test]
fn real_terminal_ctrl_c_restores_terminal_modes() {
    let repository = TempDir::new().unwrap();
    let mut terminal = Session::start(repository.path());
    terminal.exit(b"\x03");
}

#[test]
fn real_terminal_mouse_saves_title_only_record() {
    let repository = TempDir::new().unwrap();
    let path = repository.path().join("DECISIONS.md");
    let mut terminal = Session::start(repository.path());
    new_draft(&mut terminal, true);
    terminal.type_text("Title only");
    terminal.wait_for("Title only");
    terminal.send(b"\t\t\t\t\t");
    terminal.click_text("Save · Ctrl+S");
    terminal.wait_for("Saved decision");
    terminal.exit(b"q");
    let document = vrdx::document::Document::load(path).unwrap();
    assert_eq!(document.records.len(), 1);
    assert_eq!(document.records[0].title, "Title only");
    assert_eq!(document.records[0].decision, "");
}

#[test]
fn real_terminal_initialization_never_clobbers_concurrent_file() {
    let repository = TempDir::new().unwrap();
    let path = repository.path().join("DECISIONS.md");
    let mut terminal = Session::start(repository.path());
    new_draft(&mut terminal, true);
    terminal.type_text("Keep this draft");
    terminal.wait_for("Keep this draft");
    std::fs::write(&path, "Somebody created this file.\n").unwrap();
    terminal.send(b"\x13");
    terminal.wait_for("File changed on disk:");
    terminal.wait_for("Keep this draft");
    assert_eq!(
        std::fs::read_to_string(path).unwrap(),
        "Somebody created this file.\n"
    );
    terminal.send(b"\x1b");
    terminal.wait_for("VIEW");
    terminal.exit(b"q");
}

#[test]
fn real_terminal_guard_can_save_and_then_quit() {
    let repository = TempDir::new().unwrap();
    let path = repository.path().join("DECISIONS.md");
    let mut terminal = Session::start(repository.path());
    new_draft(&mut terminal, true);
    terminal.type_text("Save before quitting");
    terminal.send(b"\x11");
    terminal.wait_for("Unsaved changes");
    terminal.exit(b"s");
    assert!(
        std::fs::read_to_string(path)
            .unwrap()
            .contains("Save before quitting")
    );
}

#[test]
fn real_terminal_external_sigterm_restores_terminal_modes() {
    let repository = TempDir::new().unwrap();
    let mut terminal = Session::start(repository.path());
    let pid = terminal.child.process_id().unwrap();
    let status = std::process::Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status()
        .unwrap();
    assert!(status.success());
    terminal.exit(b"");
}

#[test]
fn command_line_help_and_errors_do_not_emit_terminal_control_codes() {
    let binary = binary();
    for argument in ["--help", "--version"] {
        let result = std::process::Command::new(&binary)
            .arg(argument)
            .output()
            .unwrap();
        assert!(result.status.success());
        assert!(!result.stdout.contains(&0x1b));
    }
    let result = std::process::Command::new(&binary)
        .arg("--unknown")
        .output()
        .unwrap();
    assert!(!result.status.success());
    let result = std::process::Command::new(&binary)
        .arg("/this/path/does/not/exist/vrdx")
        .output()
        .unwrap();
    assert!(!result.status.success());
}

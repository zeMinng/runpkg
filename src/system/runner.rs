//! Spawn a package script and stream its output. (启动脚本进程并流式读取输出)

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc::UnboundedSender;

/// A chunk of output from a running script. (运行中脚本的输出事件)
#[derive(Debug, Clone)]
pub enum ScriptEvent {
    /// One line of stdout/stderr. (一行标准输出/标准错误)
    Line(String),
    /// Process exited with the given code (`None` = terminated by signal).
    Finished(Option<i32>),
}

/// Spawn `program args...` and stream stdout/stderr lines over `tx`.
/// A `Finished` event is sent after the process exits and both pipes are drained.
pub fn spawn(program: String, args: Vec<String>, tx: UnboundedSender<ScriptEvent>) {
    let mut command = Command::new(&program);
    command.args(args);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    tokio::spawn(async move {
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(e) => {
                let _ = tx.send(ScriptEvent::Line(format!("failed to spawn `{program}`: {e}")));
                let _ = tx.send(ScriptEvent::Finished(Some(1)));
                return;
            }
        };

        let mut readers = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            readers.push(tokio::spawn(read_lines(stdout, tx.clone())));
        }
        if let Some(stderr) = child.stderr.take() {
            readers.push(tokio::spawn(read_lines(stderr, tx.clone())));
        }

        let status = child.wait().await;
        for reader in readers {
            let _ = reader.await;
        }

        let code = status.map(|s| s.code()).unwrap_or(None);
        let _ = tx.send(ScriptEvent::Finished(code));
    });
}

async fn read_lines<R>(reader: R, tx: UnboundedSender<ScriptEvent>)
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut lines = BufReader::new(reader).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let _ = tx.send(ScriptEvent::Line(line));
    }
}

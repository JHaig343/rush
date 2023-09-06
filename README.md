# rush
Shell prompt (like Bash) written in Rust.
Uses the Command structure to call /bin utilities (ls, cat, grep, etc.)
Supports redirection to file via `>` and command pipelines via `|`.
Supports filename completion, command reverse-history search using readline library

## To Run:
- `cargo run` to build and start the shell prompt
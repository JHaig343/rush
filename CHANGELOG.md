# Changelog

## 0.3.0
- command-line interaction has been improved by utilizing the `rustyline` library, which itself is a Rust hook into the readline library. You can now scroll through recent commands by pressing `UP ARROW` at the command line. Control codes like Ctrl-C and Ctrl-L (clear screen) are also supported.
- Commands are now run as separate (forked) process using std::process::Command's `spawn` functionality. commands like `vim` and `gcc` should now (mostly) work properly.
- rush now support basic file redirection; `[command] > [filename]` to redirect the output of a command to a chosen file.

To Come:
- Further use of the `rustyline` library to implement auto-completion for commands.

## 0.1.1
- added CHANGELOG.md
- removed some unused dependencies, such as structopt that were remnants of earlier experiments
- added some error-handling to the cd builtin - if cd fails for some reason ie. directory doesn't exist, trying to cd into a non-directory, etc. a descriptive error message is printed.
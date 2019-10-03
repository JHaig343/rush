## 0.1.1
- added CHANGELOG.md
- removed some unused dependencies, such as strucopt that were remnants of earlier experiments
- added some error-handling to the cd builtin - if cd fails for some reason ie. directory doesn't exist, trying to cd into a non-directory, etc. a descriptive error message is printed.
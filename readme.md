# What is this
Build tool for my out-of-tree QMK stuff.

It mainly is a fancy wrapper on top of `git` and `qmk` CLIs, this way i can have my code "easily" up to date with latest changes on QMK's codebase. Right now the tool 

As a nice extra, i made the tool auto-detect changes to its own source code and self-update. If you change on this folder and run the tool, you'll get:
```
$ qmk_build
[WARN] Detected changes on my source code, attempting to re-compile myself...
[WARN] Done. Can compile firmware now ^^

$ qmk_build
// now it actually runs //
```

# How to use
1. Install Rust ([official docs](https://www.rust-lang.org/tools/install))
2. Compile and install the executable (from this folder)
``` 
$ cargo install --path .
```
3. Describe your compilation setup on a [HJSON](https://hjson.github.io/) file.
    
    Take a look at `./schema` to see available options. You can probably setup your editor to suggest based on it. You can see how that's done on VSCode within `.vscode/`

# Contributing
Thanks to the self-update, tool should be easy to contribute to:
 * No need for manual re-compiling after changes (can't forget to)
 * Code is formatted/linted during self-update and schema is re-generated

Granted your changes compile and work as intended, feel free to PR. ie: There are no formatting rules or whatever from my end.

# Looking for my code?
This repo used to contain my code along with the tool, it's possible you followed an old link. For now (Novemeber '23) code lives on `github/elpekenin/access`

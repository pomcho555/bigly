### Bigly CLI tool

This tool can grep all files under the current directory. For example if you type `bigly "hello"`, this cli tries to find all lines across all files that includes the specified keyword `hello`. Here is the typical input and output. 

INPUT

```sh
bigly "hello"
```

OUTPUT

```sh
bigly "just"                                                                                                            ✱
CLAUDE.md
16:The project uses `just` as a command runner. Key commands from the justfile:
21:just run-tests              # Run all tests
```

NOTE: Without double quote `"` should work

Here is the common sub commands and the options

```
--help: show help of this cli

--version: show version

--file: target a specific file
```

# minigrep

## Case-sensitive

Two available options:

- Command line argument

```bash
cargo run to poem.txt I # case-sensitive
cargo run to poem.txt i # case-insensitive
```

- Environment variable

Bash example:

```bash
export CASE_INSENSITIVE=1
```

The command line argument takes precedence over the environment variable.


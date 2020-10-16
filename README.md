# tq

Command line TOML processor

## Example File Inputs

```sh
# With standard input / bash piping
$ cat Cargo.toml | tq '.'
# With file argument
$ tq -f Cargo.toml '.'
$ tq --file Cargo.toml '.'
```

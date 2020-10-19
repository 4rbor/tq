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

## Current Missing Features

Today only the basics of `tq` exist, but contributions are welcome! What's next?

- [Basic filters](https://stedolan.github.io/jq/manual/#Basicfilters)
  - Today, only the identity filter and object identifier index filter exist, the others still need to be implemented.
- [Built in Operators and Functions](https://stedolan.github.io/jq/manual/#Builtinoperatorsandfunctions)
- [Conditionals and Comparisons](https://stedolan.github.io/jq/manual/#ConditionalsandComparisons)
- [Regular Expressions](https://stedolan.github.io/jq/manual/#RegularexpressionsPCRE)
- [Math](https://stedolan.github.io/jq/manual/#Math)

## Installation

Eventually, after several of the currently missing features are implemented
we may try to get this library into the [homebrew-core](https://github.com/Homebrew/homebrew-core)
formulae, but for now you can use our custom tap.

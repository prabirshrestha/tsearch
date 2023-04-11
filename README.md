# tsearch

Search codebase using treesitter query syntax.

Inspired by https://johnaustin.io/articles/2022/blazing-fast-structural-search-for-c-sharp-in-rust.

# Supported Languages

* Typescript & TSX
* Rust

# CLI

```sh
tsearch -q -p path_to_code -- "((identifier) @name (#eq? @name \"App\"))"
```

# VIM Plugin

Requires Vim8+. Neovim not supported.

```vim
Plug 'prabirshrestha/tsearch'
```

To start searching use `:TSearch ((identifier) @name (#eq? @name "App"))`.
To cancel search use `:TSearchCancel`.

# Development

```sh
cargo run -- -q -p d:\path_to_code -- "((identifier) @name (#eq? @name \"App\"))"
```

# LICENSE

MIT

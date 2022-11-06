# Mirrors

Easily add multiple remotes to Git repositories.

### Build

Install Rust: https://www.rust-lang.org/learn/get-started

```sh
cargo build --release
export PATH="$PATH:./target/release"
```

### Usage

Create `mirrors.json` in repository root:

```json
{
  "primary": "https://github.com/username/repo.git",
  "mirrors": [
    "https://gitlab.com/username/mirror1.git",
    "https://gitlab.com/username/mirror2.git"
  ]
}
```

`primary` will be push+fetch, `mirrors` will be fetch only.

## Licensing

Mirrors is released under the [MIT License](LICENSE).

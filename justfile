root := `git rev-parse --show-toplevel`

check:
    cargo check

build:
    cargo build

commit:
    cd docs && mdbook build
    cargo doc
    # pre-commit --run
    lazygit

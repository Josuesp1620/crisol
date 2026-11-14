# Contributing to Crisol

## Set up

Every toolchain runs in Docker; nothing is installed on the host.

```bash
make images
make test
make lint
```

GPUI's Windows backend is checked natively on GitHub Actions for every push and pull request.

## Code rules

The rules are in [CLAUDE.md](CLAUDE.md): object oriented code, single responsibility per
file, type and method, descriptive names, settings and texts in one place, a test for every
rule.

## Commits and pull requests

- One logical change per commit, with a message that says what changed and why.
- Branch from `main`; keep `make test` and `make lint` green.
- User-visible changes get an entry in `CHANGELOG.md` under `Unreleased`.

## Releases

1. `make version VERSION=x.y.z`
2. Move the `Unreleased` entries of `CHANGELOG.md` under `## [x.y.z] - YYYY-MM-DD`.
3. Commit, then `git tag vx.y.z && git push origin main vx.y.z`.
4. GitHub Actions builds the Windows zip and publishes the release.

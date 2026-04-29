<!-- Thanks for the PR! Sign off your commits with `git commit -s`. -->

## Summary

<!-- 1–3 bullets -->

## Type

- [ ] Bug fix
- [ ] Feature
- [ ] New scaffold (please fill the section below)
- [ ] Refactor / chore

## New scaffold checklist

- [ ] `id` is unique and kebab-case
- [ ] `risk` set honestly
- [ ] No scope touches a message DB, key file, or user-authored content
- [ ] Tested matching on a real machine; screenshot attached
- [ ] `cargo run -p diskwise-scaffold-lint -- scaffolds/<id>.toml` passes

## Test plan

- [ ] `cargo test --workspace`
- [ ] `pnpm tauri dev` smoke-tested locally

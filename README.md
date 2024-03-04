<a href="https://aptos.dev">
 <img width="100%" src="./.assets/aptos_banner.png" alt="Aptos Banner" />
</a>

---

This project is forked from [Aptos Move](https://github.com/aptos-labs/aptos-core) to avoid multi-standard of move language.

## Validate the changes

```shell

cargo test \
 --package move-vm-integration-tests \
 --package move-cli \
 --package move-unit-test \
 --package move-compiler \
 --package move-analyzer \
 --package move-stdlib \
 --package move-package \
 --package move-binary-format \
 --package move-bytecode-verifier \
 -- --skip prove

```

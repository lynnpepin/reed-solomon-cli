# reed-solomon-cli

A simple CLI wrapper for [`reed-solomon`](https://crates.io/crates/reed-solomon).

Usage:

```
rscli encode -i lorem.txt -o encoded.bin
rscli decode -i encoded.bin    -o restored.txt
```

Loads the entire `some_input.txt` and doesn't work with large files.

TODOs:

- [ ] unit tests are a good idea
- [ ] love your users; don't chunk manually

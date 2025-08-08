# RPlate

A simple command line utility written in Rust for creating directory structures and files from text-based templates.

## Template format

```text
/parentDirectory
  /childDirectory
    myfile1.txt
    myfile2.txt
```

## Running

```bash
cargo run -- mytemplatefile.txt
```
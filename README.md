# split-csv


## Split large CSV file into smaller CSV files

## Syntax
    $ ./split-csv [in-file] [out-file-name] [max-lines-per-file]

##
```shell
cargo run -- ~/in-file.csv outfile 200000
```

```shell
target/debug/split-csv ~/in-file.csv outfile 200000
```

# Compilation
compilation is done with:
```sh
cargo build --release
```
and run with either
```sh
cargo run --release <original_photos_root_folder> <output_folder>
./target/release/jpg_uncluster <original_photos_root_folder> <output_folder>
```
notes:
- note that if `<output_folder>` does not need to be created. It will be created as long as all subfolders are 
- removing the `--release` flag will include the debuginfo in the binary, and not run the compilation with the hightest optimaliastion. 
- You can install [Rust](https://www.rust-lang.org/) and cargo with [rustup](https://rustup.rs/) 


messuring speed was done with
```sh
time ./target/release/jpg_uncluster <infolder> <outfolder>
```



# Motivation
This was a task from our teacher to sort out pictures from messed up folder to a structure by year and then name the pictures by size. This was originally done in bash, thow i made it with [Rust](https://www.rust-lang.org/). 
There are not tests as i don't have pictures that are tanken the same seconds and the code is first draft. Would be quite easy to set up once you have the pictures. 

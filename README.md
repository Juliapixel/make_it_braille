# make_it_braille

[![crates.io](https://img.shields.io/crates/v/make_it_braille.svg)](https://crates.io/crates/make_it_braille)
[![Documentation](https://docs.rs/make_it_braille/badge.svg)](https://docs.rs/make_it_braille)

this crate provides methods to create, manipulate and output images made out
of unicode braille characters such as `⠝`.

## Basic Usage
```rust no_run
use image::imageops::FilterType;
use make_it_braille::{BrailleImg, dithering::Sierra2Row};

let mut img = image::open("image.png").unwrap();
img = img.resize_exact(64, 64, FilterType::Triangle);

let mut img = BrailleImg::from_image(img, Sierra2Row, false);

println!("{}", img.as_str(true, true));
```

## Executable
makes things braille

install with `cargo install -F bin --locked --git "https://github.com/Juliapixel/make_it_braille.git"`

do `make_it_braille -h` to get a short help

### Example

`make_it_braille --width 80 "https://upload.wikimedia.org/wikipedia/en/9/9a/Trollface_non-free.png"`

```
⠁⠁⠁⠁⠁⠁⠁⠁⠁⣀⣠⣠⣄⣤⣠⣄⠤⣠⣀⡄⣠⣀⣀⣀⣀⣀⣀⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⣰⣾⣿⣿⠿⣋⡭⢶⣚⣻⣭⣭⣯⣽⣙⣛⣳⠷⣾⢖⣟⣳⣺⣖⣶⣤⣄⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⢀⣾⣿⣿⣫⡵⣻⢕⣫⣽⣶⣷⣶⣶⣦⡻⣿⣿⣿⣿⣿⢭⣭⣭⣭⣭⡛⢿⣿⣿⣦⠁⠁⠁⠁
⠁⠁⠁⠁⢀⣾⣿⣿⣿⣿⣾⣵⠟⠛⠉⠉⠉⡙⠛⠿⣿⣿⣿⣿⢿⣿⣸⣿⠿⠿⢿⣿⣾⣿⣿⣿⡄⠁⠁⠁
⠁⠁⢀⢤⣨⣏⠿⡻⣿⣈⢿⣁⢐⣀⣀⣀⣀⠛⠻⠦⠁⣻⣷⣿⠛⠛⠉⠁⠁⣀⣀⣀⣹⣕⣋⣛⣛⢦⡀⠁
⠁⣰⢵⡿⢁⣤⣶⢤⣤⣉⠙⠛⠛⠋⣁⣴⣿⣿⣷⣦⣾⣿⣿⣿⣿⡆⢰⣿⣿⣿⠿⡿⠟⠛⠛⠛⢭⢳⡝⠁
⠰⣟⢿⠁⡾⠟⠃⢀⡉⠛⠿⣿⣿⣿⡿⢿⠿⠟⡟⠛⢛⣿⡿⣿⣿⣷⣄⠙⠻⣿⣦⣤⣤⡟⢻⣿⣾⢫⡏⠁
⠁⢻⡹⣇⢹⣶⣦⠘⠿⣷⣦⡀⣈⡉⠛⠻⠿⣿⡄⢻⣍⣁⣉⣿⡯⡿⠟⢀⣤⣬⣝⣻⡿⠃⠁⢹⣗⣫⠆⠁
⠁⠁⠙⢺⣽⣿⣿⣦⠁⢄⡉⠁⠘⠿⢿⣶⣦⠄⣈⣉⡙⠛⠛⠻⠶⠤⠶⠿⠛⠛⠋⣁⡀⢤⠁⢸⣿⡇⠁⠁
⠁⠁⠁⠁⠻⣿⣿⣿⣷⡌⠻⠇⣠⣤⣀⠈⠉⠁⠻⠿⠿⠿⠆⢸⡶⡶⠁⠶⠿⠷⠁⠿⠁⠈⠁⢸⣿⡅⠁⠁
⠁⠁⠁⠁⠁⠹⣿⣿⣿⣿⣦⣄⠛⢿⣿⣿⠃⣰⣤⣄⣀⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⢸⣿⡆⠁⠁
⠁⠁⠁⠁⠁⠁⠈⠻⡿⠿⣿⠻⣷⣤⣈⠋⠠⢿⣿⣿⣿⡇⢸⣿⣾⡆⢰⣶⠆⢠⡶⢀⣎⠐⢠⣿⣿⠇⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠈⠓⠮⣟⡲⢯⣝⡻⢶⣦⣤⣤⣉⣀⣈⣉⣙⣀⣘⣉⣀⣉⣠⣤⣤⣾⣿⣿⣿⡇⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠉⠓⠺⢭⣗⣮⠭⡝⣛⢿⠻⠿⠯⠿⠽⠿⠿⠿⢿⣛⣭⣶⡿⢋⣿⡇⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠙⠻⠿⣷⣾⣿⣽⣯⣿⣻⣻⣻⣻⣻⣽⣭⣵⣾⣿⣿⠃⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠉⠉⠛⠛⠿⠿⠿⡿⢿⢿⡻⡿⠛⠋⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
```
note: also works with local files

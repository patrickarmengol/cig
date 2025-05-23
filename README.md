# cig

Generates an image of specified dimensions from a hex color code

## Installation

```sh
git clone https://github.com/patrickarmengol/cig.git
cd cig
cargo install --path .
```

## Usage

### Syntax

```
cig <WIDTH> <HEIGHT> <COLOR> [OUTPUT]
```

### Examples

```bash
# 100x100 red image
cig 100 100 FF0000

# 900x600 blue image with some transparency
cig 900 600 0000FFAA

# custom output filepath
cig 128 128 00FF00 textures/green_tile.png
```

## About

`cig` = _color image generator_

This is just a simple program intended to help me familiarize myself with rust, but I guess it does have some utility to quickly generate placeholder images or a flat color desktop wallpaper.

## Todo

- add explicit filetype option
- validate output file extension (if not using explicit filetype)
- support comma-delimited list of color codes for palettes
- use `owo-colors` to preview color in printed output

## License

`cig` is distributed under the terms of any of the following licenses:

- [Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [MIT](https://spdx.org/licenses/MIT.html)

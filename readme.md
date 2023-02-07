# Sitemap-Parser
Sitemap-parser is s a small rudimentary binary to convert sitemap XML files into CSV files. The purpose of this is to improve the ease of analysis of the sitemap.

This binary is quite rudimentary it has no overwriting protection and basic error handling.

## Usage
```bash
sitemap-parser -i <INPUT_FILE> -o <OUTPUT_FILE>
```

## Installation
```bash
cargo install --git https://github.com/speratus/sitemap-parser.git
```
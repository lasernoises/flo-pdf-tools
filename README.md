# flo-pdf-tools

Some command-line tools for working with PDFs. Uses the amazing
[lopdf](https://crates.io/crates/lopdf) crate to parse and rewrite PDFs.

## Installation

```sh
cargo install --git https://github.com/lasernoises/flo-pdf-tools.git
```

## Commands

### `textify`

Rewrites the PDF to be readable in a text editor while remaining valid and viewable in a PDF viewer.

This is possible because PDF is fundamentally mostly a text-based format. PDFs do however contain
streams, which can contain arbitrary data. Luckily these streams can be transformed using filters.
Typically this is used for compression, but here we use it to hex encode any streams that aren't
already valid UTF-8. Most page content streams are already valid UTF-8, but fonts and images aren't.

### `splat`

"Unpacks" a PDF into a folder, creating separate files for each PDF object.

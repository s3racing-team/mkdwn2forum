# mkdwn2forum

A tool for converting markdown files to some strange forum syntax.

## Web page
The web page is automatically deployed [here](https://s3racing-team.github.io/mkdwn2forum/) using github pages.

### Build
1. Install [trunk](https://trunkrs.dev/)
2. `cd` into the `web` directory
3. run `trunk serve --release` to build and serve the web page
4. open <https://localhost:8080>

## CLI
The cli will either read a file path from the first argument passed to it,
or if that isn't present, will try to read from `stdin`.

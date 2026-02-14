# Project Description
schelm-ores : [OpenResponses](https://www.openresponses.org/) Rust SDK.

## Layout
* [Types](./src/models/mod.rs) : Rust representation of types defined by OpenResponses
* [client](./src/client) : reqwest based HTTP client
* [justfile](./justfile) : Recipes for development workflow

## Development Instructions
Always run these commands after editing code to ensure the code base stays clean and in a
runnable state.
* `just check` : runs static checks, linting, and formats code
* `just test` : runs all test cases

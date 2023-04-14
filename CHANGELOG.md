# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2023-04-14

### Changed
 - frontend::Backend is now public
 - The difference between frontend::Backend and backend::Backend is documented


## [0.4.0] - 2023-04-13

This is a breaking release

### Changed
 - Config error is now an owned type losing its lifetime parameter.
 - The `lines` module renamed to `line`
 - The `sections` module renamed to `section`
 - `Line` struct now lives in `line::borrowed`
 - `Section` struct now lives in `section::borrowed`

### Added
 - Adds an owned version of `Line` in `line::owned`
 - Adds an owned version of `Section` in `section::owned`


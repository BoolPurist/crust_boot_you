# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Aliases:

CWD: current working directory

## [unreleased]

### Added 

- Added dry option. If true writing operation are not executed and only the write would-be steps are described.
- Subcommand to delete a template. 

### Changed 

- Paths to templates folder are shown along with their names.
- If no templates are created so far, user is told no templates were created yet
- CWD is tmp/crust_you_boot during development mode

## 0.1.1 - 2023.07.19

### Added

- Can list all names of created templates

### Removed

- No creating and deleting of one json file for the meta date of all templates.
  Will come back in different form. Every template will have its own meta data file.

## 0.1.0 - 2023.07.18

### Added

- Can copy file or content of folder withing the CWD under a name for later 
- Can copy saved file or folder into CWD 



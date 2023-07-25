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
- Templating system. Placeholders in following form "{{<some_value>?<default_value>}}" are filled while initializing.
  Values are asked by user in the terminal. If no value is provided then the default value is used instead.
- Different modes for initializing a project (only_empty, no_name_conflict, override, purge)
  - only_empty: Only copies If there are no previous files/folder inside the target folder.
  - no_name_conflict: Only copies If there are only previous files/folder
            which do not share a name of the files/folders from template's content.
  - override: Previous files/folders are overridden
            if they have a name of File/Folder from content of template.
  - purge: All previous files/folders inside target folder are deleted
            before content of template is copied into target.

### Changed 

- load-template: By default folder as CWD are not populated by template if it is not empty.
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



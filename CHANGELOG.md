# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Aliases:

CWD: current working directory

## [unreleased]

### Added 

- Added 2 ClI/environment option: ( left_delimiter, right_delimiter )
  Allows to change what determines the left and right border of a placeholder in file while initializing.
  Example: With left: %% and right: %%. Placeholder like %%aa%% are matched for replacement instead of {{aak}}.
- Errors within a file to augment are also shown with a line number
- Added CLI/environment option: ignore_placeholders in subcommand save-template.
  If true no placeholders are replaced by the template system.

### Fixed

- Now set custom placeholder option are taken into account. Before there were ignored in the release build.

## 0.2.1 - 2023.07.27

### Added 

- Allows testing application in release build within docker container.
- In release build all data/configuration is taken form the folders determined by XDG Standard

## 0.2.0 - 2023.07.26

### Added 

- Added option to let logger also print to terminal beside file logging.
- Added option to set logging level.
- Added option to supply logging filters which determine in which module logging is shown.
- Added logging to text file.
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

- By Default logging goes to text file instead of terminal. Terminal logging can be activated however.
- By Default an error message is shown by logger except if logging would not happen.
  Then is printed to stdout without the logger.
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



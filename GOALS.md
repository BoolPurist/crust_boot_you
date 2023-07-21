# Checklist 
- [ ] Prevent names with slashes for template names
- [x] List templates with their saved location

- [ ] Make places in files markable as placeholders which are filled during creation
  - [ ] Method 1 prompt on console
  - [ ]  Method 2 for filling placeholders. List in config folder with fixed key/value file. 
  - [ ] Method 3 for filling placeholders. Script API. Script can be registered to handle a placeholder

Scripts are given the following arguments:
>  $placeholder_name $template_name $cwd_of_cli_caller $content_of_file_with_placeholder

- [ ] Adjustable behaviour for copy to target. 
  - [ ] Stop if target folder is not empty
  - [ ] Stop if file found in target folder which would be overridden
  - [ ] Allow override
- [ ] Allow update on saved template.

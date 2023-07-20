# Features

### 0.1.3

- Make places in files markable as placeholders which are filled during creation
 - Method 1 prompt on console

### 0.1.4

- Method 2 for filling placeholders. List in config folder with fixed key/value file. 

### 0.1.5

- Method 3 for filling placeholders. Script API. Script can be registered to handle a placeholder

Scripts are given the following arguments:

- $placeholder_name $template_name $cwd_of_cli_caller $content_of_file_with_placeholder

### Other features

- Adjustable behaviour for copy to target. 
  - Stop if target folder is not empty
  - Stop if file found in target folder which would be overridden
  - Allow override
- Allow update on saved template.

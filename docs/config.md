# Database engine configurations

The database engine has configurations that configure settings and directory paths etc. These configurations use default values which can be changed by using a database client.

Below are all the configurations currently available:

Configuration | Description                 | Default value
------------- | --------------------------- | -------------
db_dir_path   | Path to database directory. | <path_to_executable>/../databases
logs_dir_path | Path to logs directory.     | <path_to_executable>/../logs

<path_to_executable> is the file path to the executable using the engine. For example, `<path_to_executable>/../databases` means that the `databases` directory is in the same directory where the executable using the engine is.

If you are not sure what the current configurations are set to, always check them by using a database client.

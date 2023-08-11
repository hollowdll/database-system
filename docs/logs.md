# Database engine logs

The database engine logs all events and errors to log files with `.log` file extension. Events include things like configuration changes and database operations. If the engine fails to do the logging, a log error is returned in the API request result.

Log files are stored to the logs directory configured by the engine. This directory can be changed to any directory you want by changing the configuration in a database client.

The log files can be used to track events happening in the engine and debug errors. They include timestamps for each log.

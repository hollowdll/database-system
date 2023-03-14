# CLI design

This documentation contains cli management system specific documentation and design.

Changes and additions will be added over time.

## Structure

Start program -> Loop inputs -> Read inputs -> Run commands -> Call engine core API

Connect to database manager -> run database manager specific commands
- Databases

Connect to a database -> run database specific commands
- Tables

Disconnect from database manager -> Disconnect all connected databases

Database manager handles connecting and disconnecting databases

## Top-level commands

/q -> Quit program

/help -> Show all commands

/connection status -> Displays connection status
- Database manager
- Databases


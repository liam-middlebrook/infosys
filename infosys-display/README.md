Infosys Display
===============

A Rust application to pull the current infosys stringset from a database and
write to a serial port.

## Config
You must make a settings.toml configuration file that contains the following
strings:
* `sign_path`
* `dbstring`

### Sample
```
sign_path = "/dev/ttyUSB0"
dbstring = "postgres://user:password@localhost/dbname"
```

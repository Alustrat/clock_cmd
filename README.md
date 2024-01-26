<div align="center">
  <h1><code>r-clock</code></h1>
  <strong>Rust command to display a clock in the terminal.</strong>
</div>

## Purpose
This is just a project to get my hands on Rust. It also allows me to keep track of GitHub features (repository settings/actions/packages/...).

## Installation
First, find and download the latest binary for your os from the [release page](https://github.com/Alustrat/r-clock/releases).

Once done, extract the executable from the archive. If you want to use it globally, move the file inside a folder which is included in your `$PATH` environment variable.

## Options
Here are the available options for the command:
```
~$ r-clock --help
Simple command to print a clock inside your terminal.

Usage: r-clock [OPTIONS]

Options:
  -s, --size <SIZE>          Define the size of the clock. Used as a multiplier.
  -f                         Keep the clock always refreshing.
  -t, --timezone <TIMEZONE>  Print the current time for the given timezone. If --utc is passed, this will be ignored.
  -u, --utc                  Print the current UTC time.
  -h, --help                 Print help

If no UTC nor timezone is passed, local time is used by default.
```
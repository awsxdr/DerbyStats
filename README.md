# DerbyStats

**NOTE: This project is currently in the initial development stage and is not yet ready for use**

DerbyStats is a tool for analyzing live roller derby games for use by teams and announcers.

## Usage ##

**When builds are available they will be found in the releases section**

DerbyStats is run from a terminal. Open a terminal on your computer and navigate to the directory that contains DerbyStats. You can then use the following command to start the application:

```./derbystats --scoreboardUrl {SCOREBOARD IP}:{SCOREBOARD PORT}```

For example, if the scoreboard was running on the same computer then you could use:

```./derbystats --scoreboardUrl localhost:8000```

You can then open a web browser and navigate to `http://localhost:8001/` to view the stats.

### Command line options

DerbyStats supports several options from the command line. These are:

| Option                | Short form | Description |
| --------------------- | ---------- | ----------- |
| `--scoreboardUrl`     | `-u`       | The URL of the scoreboard software to interact with. Default is 'localhost:8000' |
| `--hostPort`          | `-p`       | The port to host DerbyStats on. Default is `8001` |
| `--logLevel`          |            | The logging level to use. Valid values are 'trace', 'debug', 'info', 'warn', 'error', and 'none'. Default is 'info' |

## Building from source ##

This is currently the only way to try out DerbyStats. Remember that this is a work-in-progress and is likely to contain major bugs and missing features.

DerbyStats is built using a combination of Rust and Node. You will need both of these on your system to be able to build. For instructions on how to install these, see the following:

* [Installing Rust](https://www.rust-lang.org/learn/get-started)
* [Installing NodeJS](https://nodejs.org/) (Select LTS, not Current!)

Once those are installed, either clone this repo with your preferred git client, or download it. Then navigate to that directory in a terminal and run 

* On Windows: `.\build.bat release`
* On Linux: `./build.sh release`

## Contributing ##

At this stage of development I'm not looking for collaborators. However, I'm more than happy to discuss ideas and will be open to having collaborators in the future once things are a little more stable.


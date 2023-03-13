# RustDisco
RustDisco is a CLI utility tool to start making Discord bots in rust. The tool deploys all needed templates so that you have more time to attend disco parties!



# Installation

*Make sure Cargo (the Rust package manager) is installed!!*

Run the following command to install RustDisco! You only need to do this once.

```cargo install --git https://github.com/keiveulbugs/RustDisco```

*updating RustDisco can be done by rerunning the command.*


# Usage

You can call the CLI tool with `rustdisco`.


**Setting up a bot:**

Run the `-n` or the `--name` option followed by the name to deploy the basic bot structure. 
It will place these files in your current directory.

```rustdisco -n myfirstbot```

**Setting up commands:**

There are 5 different template commands. It can insert these commands after the bot is created or during the bots creation.
Right now it is important to have the help command still in your command vector as it will use that to determine where to insert the commands.

All commands are visible below:

| Options                           | Explanation |
|-----------------------------------|------------------------------|
| -n, --name <NAME>                 | Name of the bot to create. If this command runs, it will set up the whole bot structure |
| -d, --default <DEFAULT>           | Deploy a basic command |
| -e, --example <EXAMPLE>           | Deploy an example command |
| -r, --registration <REGISTRATION> | Deploy a registration command |
| -p, --purge <PURGE>               | Deploy a purge command |
| -s, --status <STATUS>             | Deploy a status command (can also be used to change nickname) |
| -h, --help                        | Print help |
|  -V, --version                    | Print version |


This will deploy 3 different commands in an existing bot:

```rustdisco -d ping -p purger -s statuschange```


**Setting up a bot with everything:**

```rustdisco -n bestbot -d pong -e lorem -r regbutton -p byebye -s watching -h helpme```

```rustdisco -n <name> -d <name> -e <name> -r <name> -p <name> -s <name> -h <name>```
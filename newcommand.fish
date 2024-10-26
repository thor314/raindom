#!/usr/bin/env fish
# argparse --min_args=1 -- $argv || return 1
if not test -f src/cli.rs ; echo must run from crate root && return 1 ; end
set new_arg $argv[1]

echo "adding $new_arg to subcommand cli..."

sd -f ms 'enum Subcommands \{(.*?)\}' "enum Subcommands { \$1  $new_arg($new_arg), \n}" src/cli.rs 
sd -f ms 'match self \{(.*?Subcommands::.*?)\}' "match self { \$1 Subcommands::$new_arg(c) => c.handle(), \n}" src/cli.rs

echo "#[derive(Parser, Debug)]
struct $new_arg;

impl $new_arg {
  pub fn handle(&self) { todo!() }
}" >> src/cli.rs

cargo fmt 

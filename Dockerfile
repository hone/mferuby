FROM hone/mruby-cli:14.04-rust

RUN apt-get update && apt-get install gdb -y

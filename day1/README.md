# Day 1 - 60 Days of Rust

Hi! Welcome to the 60 days of coding with Rust, day 1.

Today I spent some time to set up a Rust development environment with Neovim.
This setup is based on this nice tutorial: https://sharksforarms.dev/posts/neovim-rust/.
Note that the steps described below are not a tutorial. If you are looking into setting up your environment, I recommend checking the tutorial.

The commands below were executed on Ubuntu 22.04.

## Install Neovim from the source code

[detailed instructions](https://github.com/neovim/neovim/wiki/Building-Neovim)

```shell
# prereqs
$ sudo apt-get install ninja-build gettext \
    libtool libtool-bin autoconf automake \
    cmake g++ pkg-config unzip curl doxygen

# get the code
$ git clone https://github.com/neovim/neovim

# build and install
$ cd neovim && make CMAKE_BUILD_TYPE=RelWithDebInfo
$ sudo make install

# check the version
$ nvim --version
NVIM v0.9.0-dev-211+g44b88d8c3
Build type: RelWithDebInfo
LuaJIT 2.1.0-beta3
[...]
```

## Install rust-analizer

[detailed instructions](https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary)

```shell
$ curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz \
    | gunzip -c - > ~/.local/bin/rust-analyzer
$ chmod +x ~/.local/bin/rust-analyzer
```

## Configure Neovim for Rust

```shell
$ curl -L https://raw.githubusercontent.com/sharksforarms/neovim-rust/master/neovim-init-lsp-cmp-rust-tools.lua \
    -o ~/.config/nvim/init.lua
```

This step is skipping a lot of explanation - check the [before mentioned tutorial](https://sharksforarms.dev/posts/neovim-rust/) for more info.

## Check the configuration with some Rust code

I'm assuming Rust and its toolchain are installed - see the [installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html).

```shell
cargo new --vcs none

nvim src/main.rs
```

Start typing some code and note the autocomplete, documentation, and other features.

## Extra: Add markdown syntax highlight support

https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#marksman-linux


```shell
curl -L https://github.com/artempyanykh/marksman/releases/download/2022-10-30/marksman-linux \
    -o ~/.local/bin/marksman
chmod +x ~/.local/bin/marksman

echo "require'lspconfig'.marksman.setup{}" >> ~/.config/nvim/init.lua
```

Open a markdown file and the syntax should be highlighted.

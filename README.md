# ctrlg ⌨️

###### Press <kbd>ctrl</kbd> + <kbd>g</kbd> to jump between projects using a fuzzy finder

![demo](https://github.com/mrjones2014/ctrlg/raw/master/demo.gif)
<sup>Demo is using the [tmux integration](#tmux-integration) for floating window and [lighthaus](https://github.com/mrjones2014/lighthaus.nvim) terminal theme.</sup>

Ctrlg is a tool to quickly switch contexts to another directory, using a fuzzy finder.
If enabled (by setting `$CTRLG_TMUX` to `true`), `ctrlg` can `cd` all split panes in the current window of a `tmux` session
to the selected directory. Press <kbd>ctrl</kbd> + <kbd>g</kbd> to fuzzy find directories,
configured by globbing patterns.

By default, only `~/git/*` is searched. To change this or add additional
directories to search, see [configuration](#configuration).

## Install

### With Cargo

```sh
cargo install ctrlg
```

`cargo` can be installed via [rustup.rs](https://rustup.rs).

### With Installer Script

Do not run as root or with `sudo`, the script will ask for `sudo` if needed.

```sh
bash -c 'bash <(curl --proto "=https" --tlsv1.2 -sSf https://raw.githubusercontent.com/mrjones2014/ctrlg/master/install.bash)'
```

### Manual

1. Download the appropriate binary for your system from the [latest GitHub Release](https://github.com/mrjones2014/ctrlg/releases)
1. Rename the binary `ctrlg`
1. Make the binary executable via `chmod +x ctrlg`
1. Put the binary anywhere on your `$PATH`, such as `/usr/local/bin/ctrlg`

### Build and Install from Source

Requires `cargo`:

```sh
git clone git@github.com:mrjones2014/ctrlg.git
cd ctrlg
cargo install --path .
```

## Shell Plugin

Once the CLI is installed, you will need to set up the key binding depending on your shell.
Alternatively, you can disable the default keybind by setting `$CTRLG_NOBIND` to `true`
before running the init script, then set up your own keybind to call `_ctrlg_search_and_go`.

### Fish

```fish
echo 'ctrlg init fish | source' >> ~/.config/fish/config.fish
```

### Zsh

```zsh
echo 'eval "$(ctrlg init zsh)"' >> ~/.zshrc
```

### Bash

```bash
echo 'eval "$(ctrlg init bash)"' >> ~/.bashrc
```

## Tmux Integration

To make `ctrlg` send the `cd` command to all split panes in the current `tmux`
window, set the environment variable `CTRLG_TMUX` to `true`. You can also make the fuzzy finder
appear in a `tmux` floating window, and specify the window size, with `$CTRLG_TMUX_POPUP` and
`$CTRLG_TMUX_POPUP_ARGS`, respectively. `$CTRLG_TMUX_POPUP_ARGS` can be any window positioning
or sizing arguments accepted by `tmux popup`. `$CTRLG_TMUX_POPUP_ARGS` defaults to `-w 75% -h 75%`.

You can also define a hook function to send the `cd` command to additional `tmux` panes not in the
current window. The function must return a list of `tmux` pane IDs. The hook function is
`_ctrlg_get_related_panes`.

### Fish

```fish
set CTRLG_TMUX true
set CTRLG_TMUX_POPUP true
# IMPORTANT: quote each argument separately so that the variable is an array
set CTRLG_TMUX_POPUP_ARGS "-w" "75%" "-h" "75%"
```

### Bash or Zsh

```bash
export CTRLG_TMUX=true
export CTRLG_TMUX_POPUP=true
# for bash and zsh, quote all arguments together
export CTRLG_TMUX_POPUP_ARGS="-w 75% -h 75%"
```

## Key Bindings

| Key Binding                              | Function                                                                                                                                                                   |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>Enter</kbd>                         | `cd` to the selected directory. If `$CTRLG_TMUX` is `true`, the `cd` command is sent to all split panes in the current window.                                             |
| <kbd>Alt/Option</kbd> + <kbd>Enter</kbd> | `cd` to the selected directory, then open `$EDITOR` if defined. The `$EDITOR` command is only run in the currently active `tmux` pane, if using the `tmux` integration.    |
| <kbd>Alt/Option</kbd> + <kbd>o</kbd>     | Open `$EDITOR` to selected directory without `cd`ing the shell.                                                                                                            |
| <kbd>Ctrl</kbd> + <kbd>o</kbd>           | `cd` to selected directory _only in current `tmux` pane_, do not send `cd` command to other `tmux` panes.                                                                  |
| <kbd>Tab</kbd>                           | Insert the selected directory path to the command line, but do not execute anything. Works in Fish and zsh only, in bash, acts the same as <kbd>Ctrl</kbd> + <kbd>o</kbd>. |
| <kbd>Ctrl</kbd> + <kbd>d</kbd>           | Scroll preview up.                                                                                                                                                         |
| <kbd>Ctrl</kbd> + <kbd>f</kbd>           | Scroll preview down.                                                                                                                                                       |

## Configuration

`ctrlg` will look for a configuration file at `~/.config/ctrlg/config.yml`. The default
configuration is shown below:

```yaml
# include other configuration files into this configuration file,
# does not search recursively (e.g. you cannot `include` file from
# an already `include`d file). The `include` key is a yaml list
# of file paths to include
include: []
# configure what directories to list in the fuzzy finder
# can be any list of globbing patterns, will only show directories
# not files
search_dirs:
  - "~/git/*"
# globbing patterns of files to find for use as preview
# see below for more details on previews
preview_files:
  - "README.*"
# enable or disable the preview window
previews: true
# force using or not using `glow` for previews
# this setting takes precedence over `preview_with_bat`
# this represents the default but in an actual
# config file, this should just be `true` or `false`
preview_with_glow: (true if `glow` is installed, false otherwise)
# set the line-wrap width passed to `glow` via `glow -w`
glow_wrap_width: 80
# force using or not using `bat` for previews
# this represents the default but in an actual
# config file, this should just be `true` or `false`
preview_with_bat: (true if `bat` is installed and `glow` is NOT installed, false otherwise)
# force using or not using `exa` for preview fallback when no
# matching `preview_files` are found
# this represents the default but in an actual
# config file, this should just be `true` or `false`
preview_fallback_exa: (true if `exa` is installed, false otherwise)
# enable or disable showing git branch for directories
# which are git repositories
show_git_branch: true
# character to render between the directory name and git branch name
# you can change this to a Nerd Font symbol if you like
# such as git branch symbol: 
git_branch_separator: "■"
# customize color scheme
# see section "Color Schemes" below for more details
colors:
  # directory name color
  dir_name: "cyan"
  # git branch color
  git_branch: "247,78,39" # this is git's brand orange color
  # name of theme to use for `bat`
  # see: https://github.com/sharkdp/bat#highlighting-theme
  bat_theme: "ansi"
```

### Previews

Previews, if enabled, are generated by rendering the first file in each directory
matching any of the specified `preview_files` globbing patterns. If a matching file
is found, it will be rendered with [bat](https://github.com/sharkdp/bat) by default
if `bat` is installed, otherwise it will be rendered with `cat`. You can force using
or not using `bat` with the `preview_with_bat` option. You can default to always
using the fallback instead of rendering a file by setting an empty list of globbing
patterns, like: `preview_files: []`.

If no matching preview files are found, the directory listing is used as the preview. By
default, directory contents are listed using [exa](https://github.com/ogham/exa) by default
if `exa` is installed, otherwise contents are listed using `ls`. You can force using or not
using `exa` as the fallback preview using the `preview_fallback_exa` option.

### Color Schemes

Colors in the config file may be specified as a named color,
a single integer corresponding to [xterm-256 color codes](https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg),
or an RGB triple of integers (e.g. `255,255,255`). If an invalid color is specified
(e.g. if you use decimals instead of integers, or an invalid named color), it will default to
white. For `xterm-256` or RGB colors to work, it must be supported by your terminal emulator.
I recommend [Kitty](https://sw.kovidgoyal.net/kitty/).

Named colors are the following:

- `"black"`
- `"red"`
- `"green"`
- `"yellow"`
- `"blue"`
- `"purple"`
- `"cyan"`
- `"white"`

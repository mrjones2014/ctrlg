<h1 align="center">Ctrlg</h1>
<h4 align="center">Press <kbd>ctrl</kbd> + <kbd>g</kbd> to jump between projects using a fuzzy finder</h4>

![demo](https://github.com/mrjones2014/ctrlg/raw/master/demo.gif)

Ctrlg is a tool to quickly switch contexts to another directory, using a fuzzy finder.
If enabled (by setting `$CTRLG_TMUX` to `true`), `ctrlg` can `cd` all split panes in the current window of a `tmux` session
to the selected directory. Press <kbd>ctrl</kbd> + <kbd>g</kbd> to fuzzy find directories,
configured by globbing patterns.

By default, only `~/git/*` is searched. To change this or add additional
directories to search, see [configuration](#configuration).

## Install

If you have `cargo` installed, you can simply run:

```
cargo install ctrlg
```

`cargo` can be installed via [rustup.rs](https://rustup.rs).

Otherwise, you can install a pre-built binary from the [latest GitHub Release](https://github.com/mrjones2014/ctrlg/releases),
rename the binary to `ctrlg`, make it executable via `chmod +x ctrlg`, then put it anywhere on your `$PATH`.

### Shell Plugin

Once the CLI is installed, you will need to set up the key binding depending on your shell.
Alternatively, you can disable the default keybind by setting `$CTRLG_NOBIND` to `true`
before running the init script, then set up your own keybind to call `_ctrlg_search_and_go`.

#### Fish

```fish
echo 'ctrlg init fish | source' >> ~/.config/fish/config.fish
```

#### Zsh

```zsh
echo 'eval "$(ctrlg init zsh)"' >> ~/.zshrc
```

#### Bash

```bash
echo 'eval "$(ctrlg init bash)"' >> ~/.bashrc
```

### Tmux Integration

To make `ctrlg` send the `cd` command to all split panes in the current `tmux`
window, set the environment variable `CTRLG_TMUX` to `true`.

### Fish

```fish
set CTRLG_TMUX true
```

### Bash or Zsh

```bash
export CTRLG_TMUX=true
```

### Key Bindings

| Key Binding                              | Function                                                                                                                                                                |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>Enter</kbd>                         | `cd` to the selected directory                                                                                                                                          |
| <kbd>Alt/Option</kbd> + <kbd>Enter</kbd> | `cd` to the selected directory, then open `$EDITOR` if defined. The `$EDITOR` command is only run in the currently active `tmux` pane, if using the `tmux` integration. |
| <kbd>Ctrl</kbd> + <kbd>o</kbd>           | `cd` to selected directory *only in current `tmux` pane*, do not send `cd` command to other `tmux` panes

## Configuration

`ctrlg` will look for a configuration file at `~/.config/ctrlg/config.yml`. The default
configuration is shown below:

```yaml
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
# force using or not using `bat` for previews
# this represents the default but in an actual
# config file, this should just be `true` or `false`
preview_with_bat: [true if `bat` is installed, false otherwise]
# force using or not using `exa` for preview fallback when no
# matching `preview_files` are found
# this represents the default but in an actual
# config file, this should just be `true` or `false`
preview_fallback_exa: [true if `exa` is installed, false otherwise]
# enable or disable showing git branch for directories
# which are git repositories
show_git_branch: true
# character to render between the directory name and git branch name
# you can change this to a Nerd Font symbol if you like
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

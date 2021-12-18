#!/usr/bin/env bash

# Sfi a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlg_tmux_sfi_all_panes {
  if test -z "$TMUX"; then
    eval "$1"
  else
    local current_pane
    current_pane="$(tmux display-message -p '#P')"
    for pane in $(tmux list-panes -F '#P'); do
      if [ "$pane" = "$current_pane" ]; then
        eval "$1"
      else
        tmux send-keys -t "$pane" "$1" Enter
      fi
    done
  fi
}

function _ctrlg_search_and_go {
  local ctrlg_selected_dir
  ctrlg_selected_dir=$(ctrlg find)
  if test -n "$ctrlg_selected_dir"; then
    if test -n "$CTRLG_TMUX"; then
      if test -z "$CTRLG_NOCLEAR"; then
        _ctrlg_tmux_sfi_all_panes "cd $ctrlg_selected_dir; clear"
      else
        _ctrlg_tmux_sfi_all_panes "cd $ctrlg_selected_dir"
      fi
    else
      cd "$ctrlg_selected_dir" || exit
      if test -z "$CTRLG_NOCLEAR"; then
        clear
      fi
    fi
  fi
}

if test -z "$CTRLG_NOBIND"; then
  bind -x '"\C-g": _ctrlg_search_and_go'
fi

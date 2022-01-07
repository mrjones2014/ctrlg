#!/usr/bin/env zsh

# Send a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlg_tmux_send_all_panes() {
  if test -z "$TMUX"; then
    eval "$1"
  else
    local current_pane="$(tmux display-message -p '#P')"
    for pane in $(tmux list-panes -F '#P'); do
      if [ "$pane" = "$current_pane" ]; then
        eval "$1"
      else
        tmux send-keys -t "$pane" "  $1" Enter
      fi
    done
  fi
}

function _ctrlg_search_and_go() {
  local ctrlg_output="$(ctrlg find)"
  local ctrlg_selected_dir=${ctrlg_output/"ctrlg_edit:"/}
  if test -n "$ctrlg_selected_dir"; then
    if [ "$CTRLG_TMUX" = "true" ]; then
      _ctrlg_tmux_send_all_panes "cd $ctrlg_selected_dir && zle reset-prompt && clear"
    else
      cd "$ctrlg_selected_dir" || exit
      zle reset-prompt
      clear
    fi

    if [[ "$ctrlg_output" = ctrlg_edit:* ]]; then
      $EDITOR
    fi
  fi
}

zle -N _ctrlg_search_and_go

if test -z "$CTRLG_NOBIND"; then
  bindkey '^g' _ctrlg_search_and_go
fi

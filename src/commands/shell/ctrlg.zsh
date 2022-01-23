#!/usr/bin/env zsh

# Send a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlg_tmux_send_all_panes() {
  if test -z "$TMUX" || test -z "$CTRLG_TMUX"; then
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

function _ctrlg_popup() {
  if [ "$CTRLG_TMUX_POPUP" = "true" ] && [ "$TMUX" != "" ]; then
    fifo="${TMPDIR:-/tmp/}/_ctrlg_fifo"
    rm -f "$fifo"
    mkfifo "$fifo"
    popup_args="${CTRLG_TMUX_POPUP_ARGS:-"-w 75\% -h 75\%"}"
    tmux popup -E $popup_args "ctrlg find > $fifo" &
    cat "$fifo"
    rm -f "$fifo"
  else
    ctrlg find
  fi
}

function _ctrlg_search_and_go() {
  local ctrlg_output
  ctrlg_output="$(_ctrlg_popup)"
  local ctrlg_selected_dir
  ctrlg_selected_dir=${ctrlg_output/"ctrlg_edit:"/}
  ctrlg_selected_dir=${ctrlg_selected_dir/"ctrlg_notmux:"/}
  ctrlg_selected_dir=${ctrlg_selected_dir/"ctrlg_insert:"/}
  ctrlg_selected_dir=${ctrlg_selected_dir/"ctrlg_pushd:"/}

  if test -z "$ctrlg_selected_dir"; then
    return
  fi

  if [[ "$ctrlg_output" = ctrlg_insert:* ]]; then
    LBUFFER="$ctrlg_selected_dir"
  elif [[ "$ctrlg_output" = ctrlg_pushd:* ]]; then
    if test -z "$EDITOR"; then
      echo "\$EDITOR is not defined."
      zle reset-prompt
      return
    fi
    pushd "$ctrlg_selected_dir" && $EDITOR && popd
  elif [[ "$ctrlg_output" = ctrlg_notmux:* ]]; then
    cd "$ctrlg_selected_dir"
  else
    _ctrlg_tmux_send_all_panes "cd $ctrlg_selected_dir && clear"
    if [[ "$ctrlg_output" = ctrlg_edit:* ]]; then
      if test -z "$EDITOR"; then
        echo "\$EDITOR is not defined."
        zle reset-prompt
        return
      fi
      $EDITOR
    fi
  fi

  zle reset-prompt
}

zle -N _ctrlg_search_and_go

if test -z "$CTRLG_NOBIND"; then
  bindkey '^g' _ctrlg_search_and_go
fi

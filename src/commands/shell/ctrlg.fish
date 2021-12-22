# Send a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlg_tmux_send_all_panes
    if test -z "$TMUX"
        eval "$argv"
    else
        set -l current_pane (tmux display-message -p '#P')
        for pane in (tmux list-panes -F '#P')
            if [ "$pane" = "$current_pane" ]
                eval "$argv"
            else
                tmux send-keys -t "$pane" "  $argv" Enter
            end
        end
    end
end

function _ctrlg_search_and_go
    set -l ctrlg_selected_dir (ctrlg find)
    if test -n "$ctrlg_selected_dir"
        if [ "$CTRLG_TMUX" = true ]
            _ctrlg_tmux_send_all_panes "cd $ctrlg_selected_dir && commandline -f repaint && clear"
        else
            cd "$ctrlg_selected_dir"
            commandline -f repaint
            clear
        end
    end
end

if test -z "$CTRLG_NOBIND"
    bind \a _ctrlg_search_and_go
end

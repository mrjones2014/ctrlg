# Send a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlk_tmux_send_all_panes
    if test -z "$TMUX"
        eval "$argv"
    else
        set -l current_pane (tmux display-message -p '#P')
        for pane in (tmux list-panes -F '#P')
            if [ "$pane" = "$current_pane" ]
                eval "$argv"
            else
                tmux send-keys -t "$pane" "$argv" Enter
            end
        end
    end
end

function _ctrlk_search_and_go
    set ctrlk_selected_dir (ctrlk find)
    if test -n "$ctrlk_selected_dir"
        if test -n "$CTRLK_TMUX"
            if test -z "$CTRLK_NOCLEAR"
                _ctrlk_tmux_send_all_panes "cd $ctrlk_selected_dir; clear"
            else
                _ctrlk_tmux_send_all_panes "cd $ctrlk_selected_dir"
            end
        else
            cd "$ctrlk_selected_dir"
            if test -z "$CTRLK_NOCLEAR"
                clear
            end
        end
    end
end

if test -z "$CTRLK_NOBIND"
    bind \ck _ctrlk_search_and_go
end

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
                tmux send-keys -t "$pane" "$cmd" Enter
            end
        end
    end
end

function _ctrlk_search_and_go
    set -l ctrlk_selected_dir (ctrlk go)
    if test -n "$ctrlk_selected_dir"
        if [ "$CTRLK_TMUX" = true ] || [ "$CTRLK_TMUX" = true ]
            _ctrlk_tmux_send_all_panes "cd $ctrlk_selected_dir"
        else
            cd "$ctrlk_selected_dir"
        end
    end
end

if test -z "$CTRLK_NOBIND"
    bind \ck _ctrlk_search_and_go
end

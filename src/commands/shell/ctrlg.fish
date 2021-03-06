# Send a single command to all panes without
# having to toggle on and off the
# synchronize-panes option manually
function _ctrlg_tmux_send_all_panes
    if test -z "$TMUX" || test -z "$CTRLG_TMUX"
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
        if type _ctrlg_get_related_panes >/dev/null
            for pane in (_ctrlg_get_related_panes || "")
                tmux send-keys -t "$pane" "  $argv" Enter
            end
        end
    end
end

function _ctrlg_popup
    if [ "$CTRLG_TMUX_POPUP" = true ] && [ "$TMUX" != "" ]
        set -l fifo (set -q TMPDIR && echo "$TMPDIR" || echo "/tmp/")
        set -l fifo "$fifo/_ctrlg_fifo"
        rm -f "$fifo"
        mkfifo "$fifo"
        if [ "$CTRLG_TMUX_POPUP_ARGS" = "" ]
            tmux popup -E -w 75% -h 75% "ctrlg find > $fifo" &
        else
            tmux popup -E $CTRLG_TMUX_POPUP_ARGS "ctrlg find > $fifo" &
        end
        cat "$fifo"
        rm -rf "$fifo"
    else
        ctrlg find
    end
end

function _ctrlg_search_and_go
    set -l ctrlg_output (_ctrlg_popup)
    set -l ctrlg_selected_dir (string replace "ctrlg_edit:" "" "$ctrlg_output")
    set -l ctrlg_selected_dir (string replace "ctrlg_notmux:" "" "$ctrlg_selected_dir")
    set -l ctrlg_selected_dir (string replace "ctrlg_insert:" "" "$ctrlg_selected_dir")
    set -l ctrlg_selected_dir (string replace "ctrlg_pushd:" "" "$ctrlg_selected_dir")
    echo "$ctrlg_selected_dir"

    if test -z "$ctrlg_selected_dir"
        commandline -f repaint
        return
    end

    if string match -q -- "ctrlg_insert:*" "$ctrlg_output"
        commandline -r "$ctrlg_selected_dir"
    else if string match -q -- "ctrlg_pushd:*" "$ctrlg_output"
        if test -z "$EDITOR"
            echo "\$EDITOR is not defined."
            commandline -f repaint
            return
        end
        pushd "$ctrlg_selected_dir" && $EDITOR && popd
    else if string match -q -- "ctrlg_notmux:*" "$ctrlg_output"
        cd "$ctrlg_selected_dir"
        clear
    else
        _ctrlg_tmux_send_all_panes "cd $ctrlg_selected_dir && commandline -f repaint && clear"
        if string match -q -- "ctrlg_edit:*" "$ctrlg_output"
            $EDITOR
        end
    end

    commandline -f repaint
end

if test -z "$CTRLG_NOBIND"
    bind \a _ctrlg_search_and_go
    for mode in insert default normal
        bind -M $mode \a _ctrlg_search_and_go
    end
end

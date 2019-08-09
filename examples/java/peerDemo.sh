#!/bin/sh

tmux new-session -d -s foo 'cd ./../../bin && java example'
tmux split-window -v -t 0 'cd ./../../bin && java example --boot-nodes -IW4QKaKpM5ljLpEEuFjcmoqFVYpY2PVGigNX3vFWzJzfjESWmltcztnrgKP8hLHKShBZTd2lIfjpwCiZCtK8GjPQq4DgmlwhH8AAAGDdGNwgiMog3VkcIIjKIlzZWNwMjU2azGhA7mA0yD2yMhLDZ2cHtQCe-2xhLrBmcCM2Eg9jYWDFqk5 --listen-address 127.0.0.1 --port 9001 --datadir /tmp/.artemis'
tmux select-layout tile
tmux rename-window 'the dude abides'
tmux attach-session -d
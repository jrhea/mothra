#!/bin/sh

tmux new-session -d -s foo 'dotnet ./../../bin/dotnet/Example.dll'
tmux split-window -v -t 0 'dotnet ./../../bin/dotnet/Example.dll -- --boot-nodes $(cat ~/.mothra/network/enr.dat) --listen-address 127.0.0.1 --port 9001 --datadir /tmp/.artemis'
tmux select-layout tile 
tmux rename-window 'the dude abides'
tmux attach-session -d

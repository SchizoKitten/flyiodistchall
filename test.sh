#!/bin/bash

x=$@

b=./target/debug/flyiodistchall

case $x in
    1) y="test -w echo --bin $b --node-count 1 --time-limit 10";;
    2) y="test -w unique-ids --bin $b --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition";;
    3a) y="test -w broadcast --bin $b --node-count 1 --time-limit 20 --rate 10";;
    3b) y="test -w broadcast --bin $b --node-count 5 --time-limit 20 --rate 10";;
    3c) y="test -w broadcast --bin $b --node-count 5 --time-limit 20 --rate 10 --nemesis partition";;
    3d) y="test -w broadcast --bin $b --node-count 25 --time-limit 20 --rate 100 --latency 100";;
    3e) y="test -w broadcast --bin $b --node-count 25 --time-limit 20 --rate 100 --latency 100";;
esac

exec ./maelstrom/maelstrom $y

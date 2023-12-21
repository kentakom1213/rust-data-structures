#!/bin/zsh

map="src/map.rs"
node="src/node.rs"
printutil="src/print_util.rs"
iterator="src/iterator.rs"

case $1 in
    "map" ) echo 'mod node {'
            cat $node | sed 's/^/    /g'
            echo '}\n'
            echo 'mod print_util {'
            cat $printutil | sed 's/^/    /g'
            echo '}\n'
            echo 'mod iterator {'
            cat $iterator | sed 's/^/    /g'
            echo '}\n'
            echo 'mod map {'
            cat $map | sed 's/^/    /g'
            echo '}' ;;
    * ) ;;
esac

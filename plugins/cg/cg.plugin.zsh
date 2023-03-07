#!/usr/bin/env zsh

function generate() {
    emulate -L zsh

    ret=$(echo $* | cg)
    if [[ $? -ne 0 ]]; then
        return 1
    fi

    print -z $ret
}

type cg &> /dev/null
if [[ $? -ne 0 ]]; then
    echo "cg not found. please install it through https://github.com/DarkHighness/ChatGPTCommandlineGeneration"
    return
fi

alias g="generate $1"
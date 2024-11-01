#!/bin/bash

clear

# frames
frame1="
    N     
   /|\\     
   / \\     
"

frame2="
   \ N/     
    |     
   |  |     
"

frame3="
      N     
     /|\\     
    /  \\     
"

frame4="
    \N/      
     |     
   |  |     
"

frame5="
    \N/      
    |     
   |  |     
"

# ANSI escape codes for cursor movement
position() {
  printf "\033[%d;%dH" $1 $2
}

# clear previous frame
clear_frame() {
    position $1 $2
    echo -e "        "
    position $((1+$1)) $2
    echo -e "        "
    position $((2+$1)) $2
    echo -e "        "
    position $((3+$1)) $2
    echo -e "        "
    position $((4+$1)) $2
    echo -e "        "
    position $((5+$1)) $2
    echo -e "        "
}

# main animation loop
row=10
col=20

while true; do
    # Frame 1
    clear_frame $row $col
    position $row $col
    echo -e "$frame1"
    sleep 0.2

    # Frame 2
    clear_frame $row $col
    position $row $col
    echo -e "$frame2"
    sleep 0.2

    # Frame 3
    clear_frame $row $col
    position $row $col
    echo -e "$frame3"
    sleep 0.2

    # Frame 2
    clear_frame $row $col
    position $row $col
    echo -e "$frame2"
    sleep 0.2

    # Frame 1
    clear_frame $row $col
    position $row $col
    echo -e "$frame1"
    sleep 0.2

    # Frame 2
    clear_frame $row $col
    position $row $col
    echo -e "$frame2"
    sleep 0.2

    # Frame 3
    clear_frame $row $col
    position $row $col
    echo -e "$frame3"
    sleep 0.2

    # Frame 4
    clear_frame $row $col
    position $row $col
    echo -e "$frame4"
    sleep 0.2

      # Frame 5
    clear_frame $row $col
    position $row $col
    echo -e "$frame5"
    sleep 0.2

    # Frame 4
    clear_frame $row $col
    position $row $col
    echo -e "$frame4"
    sleep 0.2

      # Frame 5
    clear_frame $row $col
    position $row $col
    echo -e "$frame5"
    sleep 0.2

    # Frame 4
    clear_frame $row $col
    position $row $col
    echo -e "$frame4"
    sleep 0.2

      # Frame 5
    clear_frame $row $col
    position $row $col
    echo -e "$frame5"
    sleep 0.2
done
### *uci


1. #### go
start calculating on the current position set up with the "position" command.
There are a number of commands that can follow this command, all will be sent in the same string: 

    * `go wtime <x>`


2. ### position
Example: [`position startpos`, `position fen <FEN_STRING>`] moves <move1> ... <move2>
<br/>
Set up the position described in fenstring on the internal board and and play the moves on the internal chess board.
If the game was played from the start position the string "startpos" will be sent.


#[References]
1. [UCI Protocol](https://backscattering.de/chess/uci/):
2. [UCI Protocol Specification](https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf)
3. 
### Possible Improvements
1. Refactor BoardState so that some of the methods in are only on the Player (generic)
    i. This should reduce the number of if/elses
    ii. Use Generics with Traits e.g. Generic P with Trait Player might have (north, south, east, west methods for masks, get_castling, pawn_attacks, generate_movement e.t.c)
2. Write tests everywhere
3. Performance optimizations (inlining where necessary)
4. Properly implement Display traits where they're currently used (why are you printing inside Display? :eyes)
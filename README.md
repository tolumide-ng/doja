### Possible Improvements (TODOS!)
1. Refactor BoardState so that some of the methods in are only on the Player (generic)
    i. This should reduce the number of if/elses
    ii. Use Generics with Traits e.g. Generic P with Trait Player might have (north, south, east, west methods for masks, get_castling, pawn_attacks, generate_movement e.t.c)
2. Write tests everywhere
3. Performance optimizations (inlining where necessary)
4. Properly implement Display traits where they're currently used (why are you printing inside Display? :eyes)
5. Include the captured piece in BitMove
    i. This makes it easier to get the captured piece when scoring(score_move) the move
    ii. Or when trying to pop the piece from the board during `make_move`
    iii. If this is implemented, we can easily implement a [std::ord::Cmp trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html) which makes it easier to sort the mvList without creating a new one
6. The North, South, East, West, NE, SE, NW, SW value should be implemented on the square enum






### Some used

1. [Principal Variation Search](https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code)
2. Late Move Reductions
3. [Null-Move Forward Prunning](https://web.archive.org/web/20071031095933/http://www.brucemo.com/compchess/programming/nullmove.htm)
4. [Aspiration Windows](https://web.archive.org/web/20071031095918/http://www.brucemo.com/compchess/programming/aspiration.htm)



### References
1. [A port of Maksim Korzh's work](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs)
2. [BlueFever Software](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg)
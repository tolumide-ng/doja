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
5. [Transposition Table](https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm)



### References
1. [A port of Maksim Korzh's work](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs)
2. [BlueFever Software](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg)



### TO Read/Re-read
1. [Umko Chess Program](https://ev.fe.uni-lj.si/3-2011/Boskovic.pdf)
2. [Fully Distributed Chess Program](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=6b75facdf4608cbd798092ec6eb5436b2209e361)
3. [Computer Chess Indexing](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=02f623a38001a3288281c742f86c4775e446c33f)
5. [Searchiing for Chess](https://webdocs.cs.ualberta.ca/~tony/TechnicalReports/TR87-6.pdf)
6. [Genetic Algorithm for optimising chess position scoring](https://cs.uef.fi/pub/Theses/2004_MSc_Aksenov_Petr.pdf)
7. [Paralle Search of strong ordered game trees](https://dl.acm.org/doi/pdf/10.1145/356893.356895) (*****)

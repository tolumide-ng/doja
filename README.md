## "The devil is in the details"

> [!NOTE]
> This chess engine is currently named 'dojá' but there are chances that this might change in the nearest future
> The current NNUE implementation currently supports only AVX2

> [!Warning]
> This project is still a work in progress


### Some Optimizations + Features:
- [x]  AlphaBeta Search 
- [ ] Quiescence Search
    - [ ] Delta Prunning
    - [x] Standing Pat
- [x] Late Move Reduction (LMR)
- [x] MVV_LVA (Most Viable Victim -- Least Viable Attacker)
- [ ] Move Ordering
    - [ ] MovePicker
    - [x] Capture History(CapHist)
    - [x] Killer Moves
    - [x] History Moves
    - [x] [CounterMove History](https://www.chessprogramming.org/Countermove_Heuristic)
    - [ ] Follow-Up History (FUH) tables
    - [ ] [Refutation Table](https://www.chessprogramming.org/Refutation_Table)
    - [x] Continuation History
- [x] Transposition Table
- [x] Null Move forward Pruning
- [x] Principal Variation Node
- [x] Aspiration Window
- [x] Iterative Deepening
- [x] PV-Table (Principal Variation Table)
- [ ] Futility Pruning
- [x] [Repetitions](https://www.chessprogramming.org/Repetitions)
- [ ] Extensions
    - [ ] Singular extensions
- [ ] Full WASM support



### How to Run this Project:
1. Todo!




> 'dojá' derives inspiration from a lot of engines, blog posts, libraries, published articles, and videos.
This list may not include every reference, but I'm grateful to everyone who has contributed in their way to fostering the Open-source community
### Credits and Acknowledgments
1. [Viridithias](https://github.com/cosmobobak/viridithas)
2. [Carp](https://github.com/dede1751/carp)
3. [Bullet](https://github.com/jw1912/bullet)
4. [A port of Maksim Korzh's work](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs)
5. [BlueFever Software](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg)
6. [PeSTO's Evaluation Function](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function)
7. [Obsidian](https://github.com/gab8192/Obsidian)
8. [Stockfish](https://github.com/official-stockfish/Stockfish)



### Some used
1. [Principal Variation Search](https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code)
2. Late Move Reductions
3. [Null-Move Forward Prunning](https://web.archive.org/web/20071031095933/http://www.brucemo.com/compchess/programming/nullmove.htm)
4. [Aspiration Windows](https://web.archive.org/web/20071031095918/http://www.brucemo.com/compchess/programming/aspiration.htm)
5. [Transposition Table](https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm)



### TO Read/Re-read
1.  [Umko Chess Program](https://ev.fe.uni-lj.si/3-2011/Boskovic.pdf)
2.  [Fully Distributed Chess Program](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=6b75facdf4608cbd798092ec6eb5436b2209e361)
3.  [Computer Chess Indexing](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=02f623a38001a3288281c742f86c4775e446c33f)
5.  [Searchiing for Chess](https://webdocs.cs.ualberta.ca/~tony/TechnicalReports/TR87-6.pdf)
6.  [Genetic Algorithm for optimising chess position scoring](https://cs.uef.fi/pub/Theses/2004_MSc_Aksenov_Petr.pdf)
7.  [Parallel Search of strong ordered game trees](https://dl.acm.org/doi/pdf/10.1145/356893.356895) (*****)
8.  [Pesto's evaluation table](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function)
9.  [Little Chess Evaluation Compedium](https://www.chessprogramming.org/images/7/70/LittleChessEvaluationCompendium.pdf) (****)
10. [History Heuristic](https://www.chessprogramming.org/History_Heuristic) (*****)


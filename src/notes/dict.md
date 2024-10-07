1. [Killer Moves: (i.e. killer_moves):](https://www.chessprogramming.org/Killer_Move)
<br />In Move ordering, killer moves usually come right after the has move, and (good) captures.
<br />        - In many positions, ther is only a small set of moves creating a threat or defending against it(threats). Those that cannot create, or defend against a threat might(should) be refuted.

2. [History Moves:](https://www.chessprogramming.org/History_Heuristic)
<br /> This is another move ordering heuristic. History moves record how successful a `move` has been across multiple search(tree) branches, regardless of the position(the state of the board. We are only concerned about the `Move` irrespective of other pieces and their positions on the board).
History Moves only occur during `Non-capturing` `cutoffs` (i.e score > alpha).
- In this project, we index the `history moves table` using [piece][to], where `piece` is the moving piece, and `to` is the target position.
- The value added is `depth`
- Values from History moves are only used for ordering `non-capturing` moves


3. [Principal Variation Table: (i.e. pv_table)](https://sites.google.com/site/tscpchess/principal-variation)
<br /> Good information on PV_TABLE can be found on the provided link in this title
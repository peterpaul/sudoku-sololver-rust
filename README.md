A Sudoku puzzle is represented by a Board.  The size of a puzzle is
expressed by `group_size`, the highest number in used in that
board. For a rectangular puzzle this equals the width and height of
the Board.

A Board contains cells and groups of cells.
A Cell can be in multiple groups.

In the default layout the cells of each row and each column form a
group.  Additionally blocks are formed from rectangles with width
`block_width` and height `block_height`. `block_width` times
`block_height` MUST be equal to `group_size`.

For example a 6 by 6 puzzle could be defined with the following
parameters:

- `group_size`: 6
- `block_width`: 2
- `block_height`: 3

Below is one solution to that puzzle.

    +-------+-------+-------+
    | 1   2 | 3   4 | 5   6 |
    |       |       |       |
    | 3   4 | 5   6 | 1   2 |
    |       |       |       |
    | 5   6 | 1   2 | 3   4 |
    +-------+-------+-------+
    | 2   1 | 4   3 | 6   5 |
    |       |       |       |
    | 4   3 | 6   5 | 2   1 |
    |       |       |       |
    | 6   5 | 2   1 | 4   3 |
    +-------+-------+-------+

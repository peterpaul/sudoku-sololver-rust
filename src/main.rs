use sudoku::Board;
use std::fs;

fn main() {
    let board = Board::from_string(&fs::read_to_string("puzzle.txt").expect("Could not read puzzle.txt"));

    println!("Puzzle to solve:");
    board.cells.print();

    let solutions = board.solve();

    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));

    println!("Solution:");
    for s in solutions {
        s.cells.print();
    }
}


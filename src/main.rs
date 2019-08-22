use sudoku::Board;
use std::fs;

fn main() {
    let puzzle = fs::read_to_string("puzzle.txt")
        .expect("Could not read puzzle.txt");
    let board = Board::from_string(&puzzle);

    println!("Puzzle to solve:");
    board.pretty_print();

    let solutions = board.solve();

    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));

    println!("Solution:");
    for s in solutions {
        s.pretty_print();
    }
}


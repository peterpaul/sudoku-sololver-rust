use sudoku::RectangularBoard;
use std::fs;

fn main() {
    let puzzle = fs::read_to_string("puzzle.txt")
        .expect("Could not read puzzle.txt");
    let board = RectangularBoard::from_string(&puzzle);

    println!("Puzzle to solve:");
    board.pretty_print();

    let solutions = board.solve();

    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));

    println!("Solution:");
    for s in solutions {
        s.pretty_print();
    }

    for x in 1..6 {
        for y in x..6 {
            if x * y < 6 {
                let board = RectangularBoard::new(x, y);
                board.pretty_print();
                let solutions = board.solve();
                println!("-> has {} solutions", solutions.len());
                let valid_solutions: Vec<&RectangularBoard> = solutions.iter()
                    .filter(|s| { s.is_valid_solution() })
                    .collect();
                println!("   and {} valid solutions", valid_solutions.len());
            }
        }
    }
}


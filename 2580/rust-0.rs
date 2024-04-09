use std::io::{ stdin, stdout, BufWriter, Read, Write };

const SIZE: usize = 9;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut sudoku = [[0; SIZE]; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            sudoku[i][j] = input.next().unwrap();
        }
    }

    set_sudoku(&mut sudoku);

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    for i in 0..SIZE {
        writeln!(
            out,
            "{}",
            sudoku[i].iter().map(usize::to_string).collect::<Vec<String>>().join(" ")
        ).unwrap();
    }
}

fn set_sudoku(sudoku: &mut [[usize; SIZE]; SIZE]) -> bool {
    for x in 0..SIZE {
        for y in 0..SIZE {
            if sudoku[x][y] == 0 {
                for num in 1..=SIZE {
                    if is_possible(sudoku, x, y, num) {
                        sudoku[x][y] = num;
                        if set_sudoku(sudoku) {
                            return true;
                        }
                        sudoku[x][y] = 0;
                    }
                }
                return false;
            }
        }
    }

    true
}

fn is_possible(sudoku: &[[usize; SIZE]; SIZE], row: usize, col: usize, num: usize) -> bool {
    for i in 0..SIZE {
        if sudoku[row][i] == num || sudoku[i][col] == num {
            return false;
        }
    }

    let start_x = (row / 3) * 3;
    let start_y = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku[start_x + i][start_y + j] == num {
                return false;
            }
        }
    }

    true
}

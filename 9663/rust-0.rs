use std::io::{ stdin, stdout, BufWriter, Write };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut cnt = 0;
    let mut board = vec![vec![false; n]; n];

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    nqueen(&mut board, 0, n, &mut cnt);

    writeln!(out, "{}", cnt).unwrap();
}

fn nqueen(board: &mut Vec<Vec<bool>>, row: usize, n: usize, cnt: &mut i32) {
    if row == n {
        *cnt += 1;
        return;
    }

    for col in 0..n {
        if able(board, row, col) {
            board[row][col] = true;
            nqueen(board, row + 1, n, cnt);
            board[row][col] = false;
        }
    }
}

fn able(board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i][col] {
            return false;
        }
    }

    let mut i = (row as i32) - 1;
    let mut j = (col as i32) - 1;

    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] {
            return false;
        }

        i -= 1;
        j -= 1;
    }

    let mut i = (row as i32) - 1;
    let mut j = (col as i32) + 1;

    while i >= 0 && j < (board.len() as i32) {
        if board[i as usize][j as usize] {
            return false;
        }

        i -= 1;
        j += 1;
    }

    true
}

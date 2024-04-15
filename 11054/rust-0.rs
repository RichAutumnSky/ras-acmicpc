use std::io::{ stdin, stdout, BufWriter, Read, Write };

const SIZE: usize = 1000;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    let n = input.next().unwrap();
    let mut arr = [0; SIZE];
    for i in 0..n {
        arr[i] = input.next().unwrap();
    }

    let result = bitonic_sequence(n, &arr);

    writeln!(out, "{}", result).unwrap();
}

fn bitonic_sequence(n: usize, arr: &[usize; SIZE]) -> usize {
    let mut dp = [0; SIZE];
    let mut dp2 = [0; SIZE];
    let mut result = 0;

    for i in 0..n {
        dp[i] = 1;
        for j in 0..i {
            if arr[j] < arr[i] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }

    for i in (0..n).rev() {
        dp2[i] = 1;
        for j in (i..n).rev() {
            if arr[j] < arr[i] && dp2[i] < dp2[j] + 1 {
                dp2[i] = dp2[j] + 1;
            }
        }
    }

    for i in 0..n {
        result = result.max(dp[i] + dp2[i] - 1);
    }

    result
}

pub fn main() {
    let digits: Vec<i32> = super::utils::get_string_from_stdio()
        .trim()
        .chars()
        .map(|val| val.to_string().parse().unwrap())
        .collect();
    part1(&digits);
    part2(&digits);
}
fn part1(digits: &[i32]) {
    let size = digits.len();
    let init_matrix = vec![digits.to_owned()];
    let mult_matrix = get_matrix(size);

    // println!("init_matrix {:?}", init_matrix);
    // println!("mult_matrix {:?}", mult_matrix);

    // Part 1
    let mut current_matrix = init_matrix.clone();
    for i in 0..100 {
        current_matrix = matrix_multiplication(&current_matrix, &mult_matrix, true);
        // println!("Ha {:?}", current_matrix);
    }
    println!("Part 1 {:?}", peek_result(&current_matrix, 0));
}

fn part2(digits: &[i32]) {
    // Part 2
    let size = digits.len();

    // Input length (real): 650
    // Part 2 input length (real): 6_500_000
    // Part 2 input offset (real): 5_971_989
    // For part 2, I can focus only at the quadrants of the matrix where there will be no negative number
    // ( 1  0  0 | 0  0  0)
    // ( 0  1  0 | 0  0  0) <--- this quadrant
    // (-1  1  1 | 0  0  0)
    // -------------------       and
    // ( 0  0  1 | 1  0  0)
    // ( 1  0  0 | 1  1  0) <--- this quadrant
    // ( 0 -1  0 | 1  1  1)
    // Therefore this `mult_matrix` can do a matrix_exp(mult_matrix, 100) but my RAM is not enough
    // so need to think of a more in-place solution
    let total_size = size * 10_000; // 6_500_000
    let offset = 5_971_989; // TODO: This value is based on real input value
    let mut current_digits: Vec<i32> = vec![0; total_size - offset + 1];
    for i in 0..=total_size - offset {
        current_digits[i] = digits[(i + offset) % size];
    }
    println!("current_digits len = {:?}", current_digits.len());

    let mut p = 0;
    while p < 100 {
        let mut new_digits = current_digits.clone();

        let mut i = total_size - 1;
        let mut current_digit = 0;
        // println!("p {:?}", p);
        while i >= offset {
            current_digit = (current_digit + current_digits[i - offset]) % 10;
            new_digits[i - offset] = current_digit;
            i -= 1;
            // println!("p {:?} i {}", p, i);
        }
        current_digits = new_digits;

        // {
        //     let mut strings: Vec<String> = vec![];
        //     for i in 0..8 {
        //         strings.push(current_digits[i].to_string());
        //     }
        //     println!("p {:?}; {:?}", p, strings.join(""));
        // }

        p += 1;
    }

    {
        let mut strings: Vec<String> = vec![];
        for i in 0..8 {
            strings.push(current_digits[i].to_string());
        }
        println!("Part 2 {:?}", strings.join(""));
    }
}

fn peek_result(mat: &[Vec<i32>], offset: usize) -> String {
    let mut strings: Vec<String> = vec![];
    for i in 0..8 {
        strings.push(mat[0][offset + i].to_string());
    }

    strings.join("")
}

/**
 * A: matrix of m x n
 * B: matrix of n x p
 *
 * result of A x B = C
 *
 * C: matrix of m x p
 *
 * NOTE: custom behavior for this problem (with modulus 10 + abs at the end)
 * */
fn matrix_multiplication(a: &[Vec<i32>], b: &[Vec<i32>], use_abs: bool) -> Vec<Vec<i32>> {
    assert_eq!(a[0].len(), b.len()); // a column must be b rows
    let a_rows = a.len();
    let b_rows = b.len();
    let b_cols = b[0].len();
    let mut result = vec![vec![0; b_cols]; a_rows];
    for i in 0..a_rows {
        for j in 0..b_cols {
            let mut c = 0;
            for k in 0..b_rows {
                c += a[i][k] * b[k][j] % 10;
            }
            if use_abs {
                result[i][j] = (c % 10).abs();
            } else {
                result[i][j] = c % 10;
            }
        }
    }
    result
}

fn get_matrix(size: usize) -> Vec<Vec<i32>> {
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
    let mut result = vec![vec![0; size]; size];
    for i in 0..size {
        // row
        for j in 0..size {
            // column
            result[i][j] = base_pattern[((i + 1) / (j + 1)) % 4];
        }
    }
    result
}

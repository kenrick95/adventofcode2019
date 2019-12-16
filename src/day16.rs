pub fn main() {
    let digits: Vec<isize> = super::utils::get_string_from_stdio()
        .trim()
        .chars()
        .map(|val| val.to_string().parse().unwrap())
        .collect();
    let size = digits.len();
    let init_matrix = vec![digits];
    let mult_matrix = get_matrix(size);

    // println!("init_matrix {:?}", init_matrix);
    // println!("mult_matrix {:?}", mult_matrix);

    let mut current_matrix = init_matrix.clone();
    for i in 0..100 {
        current_matrix = matrix_multiplication(&current_matrix, &mult_matrix);
        // println!("Ha {:?}", current_matrix);
    }
    println!("Part 1 {:?}", peek_result(&current_matrix));
    // println!(
    //     "Ha {:?}",
    //     matrix_multiplication(&init_matrix, &matrix_exp(&mult_matrix, 1))
    // );
    // println!(
    //     "Ha {:?}",
    //     matrix_multiplication(&matrix_multiplication(&init_matrix, &mult_matrix), &mult_matrix)
    // );
    // println!(
    //     "Ha {:?}",
    //     matrix_multiplication(&init_matrix, &matrix_exp(&mult_matrix, 2))
    // );
}

fn peek_result(mat: &Vec<Vec<isize>>) -> String {
    let mut strings: Vec<String> = vec![];
    for i in 0..8 {
        strings.push(mat[0][i].to_string());
    }

    strings.join("")
}

// NOTE: -_- cannot use this
fn matrix_exp(mat: &Vec<Vec<isize>>, pow: usize) -> Vec<Vec<isize>> {
    println!("pow {}", pow);
    if pow == 1 {
        mat.clone()
    } else if pow % 2 == 0 {
        let halved_matrix = matrix_exp(mat, pow / 2);
        matrix_multiplication(&halved_matrix, &halved_matrix)
    } else {
        matrix_multiplication(&matrix_exp(mat, pow - 1), mat)
    }
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
fn matrix_multiplication(a: &Vec<Vec<isize>>, b: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    assert_eq!(a[0].len(), b.len());
    let m = a.len();
    let n = b.len();
    let p = b[0].len();
    let mut result = vec![vec![0; p]; m];
    for i in 0..m {
        for j in 0..p {
            let mut c = 0;
            for k in 0..n {
                c += a[i][k] * b[k][j] % 10;
            }
            result[i][j] = (c % 10).abs(); // NOTE: -_- cannot use `matrix_exp` because of this stupid abs
        }
    }
    result
}

fn get_matrix(size: usize) -> Vec<Vec<isize>> {
    let base_pattern: Vec<isize> = vec![0, 1, 0, -1];
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

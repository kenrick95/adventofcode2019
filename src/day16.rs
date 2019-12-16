pub fn main() {
    let digits: Vec<i8> = super::utils::get_string_from_stdio()
        .trim()
        .chars()
        .map(|val| val.to_string().parse().unwrap())
        .collect();
    let size = digits.len();
    let init_matrix = vec![digits.clone()];
    let mult_matrix = get_matrix(size);

    // println!("init_matrix {:?}", init_matrix);
    // println!("mult_matrix {:?}", mult_matrix);

    // Part 1
    // let mut current_matrix = init_matrix.clone();
    // for i in 0..100 {
    //     current_matrix = matrix_multiplication(&current_matrix, &mult_matrix, true);
    //     // println!("Ha {:?}", current_matrix);
    // }
    // println!("Part 1 {:?}", peek_result(&current_matrix, 0));


    {
        // Part 2

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
        let repeat_count = 10_000;
        let mult_matrix = get_matrix(size * repeat_count);
        let mut humongous_init_matrix: Vec<Vec<i8>> = vec![vec![0; size * repeat_count]; 1];
        for i in 0..repeat_count {
            for j in 0..size {
                humongous_init_matrix[0][i * size + j] = digits[j];
            }
        }
        println!("Done 1");
        let mut current_matrix_pt2 = humongous_init_matrix.clone();
         // TODO: Something like this, but my RAM is not enough, definitely cannot use matrix 
        current_matrix_pt2 =
            matrix_multiplication(&current_matrix_pt2, &matrix_exp(&mult_matrix, 100), false);
        peek_result(&current_matrix_pt2, 5971989);
    }
}

fn peek_result(mat: &Vec<Vec<i8>>, offset: usize) -> String {
    let mut strings: Vec<String> = vec![];
    for i in 0..8 {
        strings.push(mat[0][offset + i].to_string());
    }

    strings.join("")
}

fn matrix_exp(mat: &Vec<Vec<i8>>, pow: usize) -> Vec<Vec<i8>> {
    println!("matrix_exp {}", pow);
    if pow == 1 {
        mat.to_owned()
    } else if pow % 2 == 0 {
        let halved_matrix = matrix_exp(mat, pow / 2);
        matrix_multiplication(&halved_matrix, &halved_matrix, false)
    } else {
        matrix_multiplication(&matrix_exp(mat, pow - 1), mat, false)
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
fn matrix_multiplication(
    a: &Vec<Vec<i8>>,
    b: &Vec<Vec<i8>>,
    use_abs: bool,
) -> Vec<Vec<i8>> {
    println!("matrix_multiplication");
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
            if use_abs {
                result[i][j] = (c % 10).abs();
            } else {
                result[i][j] = (c % 10);
            }
        }
    }
    result
}

fn get_matrix(size: usize) -> Vec<Vec<i8>> {
    let base_pattern: Vec<i8> = vec![0, 1, 0, -1];
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

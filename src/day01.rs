pub fn main() {
    let mut answer_pt1 = 0;
    let mut answer_pt2 = 0;
    let mut has_input = true;
    while has_input {
        match super::utils::get_number_from_stdio::<i32>() {
            Ok(input) => {
                // println!("Hello {}", input);
                let line_answer: i32 = (input / 3) - 2;
                answer_pt1 = answer_pt1 + line_answer;

                let mut result = line_answer;
                while result > 0 {
                    answer_pt2 += result;
                    result = (result / 3) - 2;
                }
            }
            Err(_) => {
                has_input = false;
            }
        };
    }
    println!("answer_pt1 {}", answer_pt1);
    println!("answer_pt2 {}", answer_pt2);
}

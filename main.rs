/*
* PROJECT: rs-project-25-05-2021
* DESCRIPTION: Given a Vec<T>, return the largest value from it.
* AUTHOR: Vahin Sharma
*/

use std::collections::HashMap;

fn solution<T: std::cmp::PartialOrd + Copy>(vector: &Vec<T>) -> T {
    let mut largest = vector[0];
    for &element in vector {
        if element > largest {largest = element;}
    }
    largest
}

fn main() {
    let question_answers: HashMap<Vec<i64>, i64> = [
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10),
        (vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 10),
        (vec![1, 5, 3, 9, 7, 6, 4, 19], 19),
        (vec![2454, 352, 635, 6352, 635, 6, 536], 6352),
        (vec![46, 436, 4563, 5, 425, 45, 245], 4563),
        (vec![-35634, -456334, -3243, 1], 1),
        (vec![1, -2, 3, -4, 5], 5),
        (vec![20, 20], 20),
        (vec![0, -1], 0),
        (vec![2 - 3, 5 - 4], 1),
    ]
    .iter().cloned().collect();

    let mut marks: u8 = 0;    
    for (question, actual_answer) in &question_answers {
        let answer = solution(question);
        if answer == *actual_answer {
            marks += 1;
            println!("Passed");
        } else {
            println!("Expected {:}, instead got {:}", actual_answer, answer);
        }
    }

    println!("\nScore: {} out of {}", marks, question_answers.keys().len())
}

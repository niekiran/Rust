/*Write a program to evaluate and assign letter grades to students 
based on their test scores using,

1)  if-else-if ladder statement.
2) match statement

Hint:  
Assign letter grades based on these criteria:
90 to 100: Grade A
80 to 89: Grade B
70 to 79: Grade C
60 to 69: Grade D
Below 60: Grade F

Expected output:
Enter student's test score: 85
The student's grade is: B
 */


use std::io;
fn main() {
    let mut input = String::new();

    println!("Enter student's test score(Valid score: 0 to 100):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let score: u32 = input
        .trim()
        .parse()
        .expect("Invalid input");
 
    //Explore different ways of implementing the same logic 
    grade_score_using_if_else_if_without_range_expr(score);
    grade_score_using_if_else_if_with_range_expr(score);
    grade_score_using_match(score);

}

//Uses if..else if..else + logical operators + comparision operators
fn grade_score_using_if_else_if_without_range_expr(score: u32) {
    if score < 60 {
        print_grade("F");
    } else if score >= 60 && score <= 69 {
        print_grade("D");
    } else if score >= 70 && score <= 79 {
        print_grade("C");
    } else if score >= 80 && score <= 89 {
        print_grade("B");
    } else if score >= 90 && score <= 100 {
        print_grade("A");
    } else {
        println!("Score cannt be higher than 100");
    }
}

//Uses if..else if..else + range expression + an Iterator's method 'contains'
fn grade_score_using_if_else_if_with_range_expr(score: u32) {
    if score < 60 {
        print_grade("F");
    } else if (60..=69).contains(&score) {
        print_grade("D");
    } else if (70..=79).contains(&score) {
        print_grade("C");
    } else if (80..=89).contains(&score) {
        print_grade("B");
    } else if (90..=100).contains(&score) {
        print_grade("A");
    } else {
        println!("Score cannt be higher than 100");
    }
}

//Uses  'matach' + range expression
fn grade_score_using_match(score: u32) {
    match score {
        0..=59 => print_grade("F"),
        60..=69 => print_grade("D"),
        70..=79 => print_grade("C"),
        80..=89 => print_grade("B"),
        90..=100 => print_grade("A"),
        _ => println!("Score cannot be higher than 100"),
    }
}

//here the argument 'grade' is of type 'String literal' also called as 'String slice' of type &str
fn print_grade(grade: &str) {
    println!("The Student's grade is : {grade}");
}

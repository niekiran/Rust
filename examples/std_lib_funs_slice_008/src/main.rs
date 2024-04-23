/*
    Manipulating array with slice methods
*/
fn main() {
    // Initialize an array with integers and reverse the order of elements
    let mut my_array = [1, 2, 555, 4, 5];
    my_array.reverse();
    println!("{:?}", my_array); 

    // Sort the array to find the largest element
    my_array.sort();
    let len = my_array.len();
    println!("Biggest number :{}", my_array[len - 1]); 

    // Initialize two arrays and concatenate them into a third array
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    let array3 = [array1, array2].concat();
    println!("{:?} + {:?} = {:?}", array1, array2, array3);  

    // Split a single array into two at a specified index
    let my_array2 = [34, 56, 7, 89, 5, 4, 3, 6, 73, 2];
    let (a, b) = my_array2.split_at(3);  // Split array my_array2 at index 3
    println!("a = {:?}", a);  // Display the first part of the split array
    println!("b = {:?}", b);  // Display the second part of the split array
}

fn main() {
    println!("Hello, arrays!");
    // construct mutable array
    let mut array = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("Array: {:#?}", array);

    // access elements of array
    println!("First element: {}", array[0]);
    println!("2nd element in array: {}", array[1]);

    // update elements of array
    array[4] = 10;
    array[2] = 15;
    println!("Last element in array: {}", array[4]);

    // find length of array
    println!("Length of array: {}", array.len());

    // sum array elements
    let sum: i32 = array.iter().sum();
    println!("Sum of array elements: {}", sum);

    // sort array elements
    array.sort();
    println!("Sorted array: {:?}", array);

    array.reverse();
    println!("Reversed array: {:?}", array);

    println!("---------1---------");
    // construct array of arrays
    let multi_array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Multi-dimensional array: {:?}", multi_array);
    println!("Multi-dimensional array: {:#?}", multi_array);
    println!("---------------2------------");
    // access elements of multi-dimensional array
    println!("First element of first array: {}", multi_array[0][0]);
    println!("array 1, element 2: {}", multi_array[0][1]);
    println!("array 2, element 1: {}", multi_array[1][0]);
    println!("array 3, element 3: {}", multi_array[2][2]);
    println!("array 1, element 3: {}", multi_array[0][2]);
}

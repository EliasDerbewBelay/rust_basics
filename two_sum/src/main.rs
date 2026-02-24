fn main() { 
    // Our array of numbers
    let numbers = [2, 7, 11, 15];
    let target = 9;

    // Call the function and store the result
    let result = two_sum(numbers, target);

    // Print the result
    println!("The indices are: ({}, {})", result.0, result.1);
}

fn two_sum(arr: [i32; 4], target: i32) -> (usize, usize) {
    // Hint: Use a nested for loop here.
    
    for i in 0..arr.len() {
       //insert code here   

       for j in (i + 1)..arr.len(){
        if arr[i] + arr[j] == target {
            return (i, j);
        }
       }         
        }
    

    // If no answer is found, just return (0, 0) as a fallback
    (0, 0)
}

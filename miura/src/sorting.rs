/*
* Several Rust implementations of popular sorting algorithms.
*/

/**
* Uses the quicksort algorithm to sort the passed array of positive integers.
*/
pub fn quicksort(a: &Vec<u32>) -> Vec<u32> {
    // empty list and list with only one element are trivially sorted
    if a.len() <= 1 {
        return a.to_vec();
    }

    /*
    * Isolate first element in the array (pivot) from the rest of the vector.
    * Note that from now on, we can assume the array to contain at least two elements.
    */
    let pivot = a[0];
    let rest = &a[1..];

    /*
    * Split up list into two lists:
    * those of the elements that are smaller and those that are greater/equal than the pivot.
    *
    * Note: I feel like this would be more elegant with closures to compute left and right.
    */
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &elem in (*rest).iter() { // iterator contains references
        if elem < pivot {
            left.push(elem);
        }
        else {
            right.push(elem);
        }
    }

    /*
    * Recursively solve the subproblems and concatenate the results.
    */
    let mut result:Vec<u32> = Vec::new();
    result.append(&mut quicksort(&left));
    result.push(pivot);
    result.append(&mut quicksort(&right));
    result
}
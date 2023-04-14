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
    * those of the elements that are smaller/equal and those that are greater than the pivot.
    */
    let mut left: Vec<u32> = (*rest).iter() // iterator over a vector to references to unsigned integers
        .filter(|&&x| x <= pivot) // the actual filtering
        /*
        * We still have a structure containing references to integers.
        * Need to turn it into a vector containing unsigned integers for the collect-call to work.
        */
        .map(|x| *x)
        .collect();
    let mut right: Vec<u32> = (*rest).iter().filter(|&&x| x > pivot).map(|x| *x).collect();

    /*
    * Recursively solve the subproblems and concatenate the results.
    */
    let mut result:Vec<u32> = Vec::new();
    result.append(&mut quicksort(&left));
    result.push(pivot);
    result.append(&mut quicksort(&right));
    result
}
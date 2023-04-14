/*
* Several Rust implementations of popular sorting algorithms.
*/

use std::collections::HashMap; // for counting occurences in counting sort

/*
* Uses the quicksort algorithm to sort the passed array of positive integers.
*
* Quicksort is a Divide-and-Conquer algorithm which splits up the passed array a
* into two shorter arrays based on a pivot element (here: the first element in a).
* The "left" subarray contains all elements smaller/equal than the pivot element
* while the right one contains all greater elements.
* These two subarrays are then recursively sorted and "inserted" left and right of the pivot.
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
        /*
        * The actual filtering.
        * Note that an iterator contains references to the type of elements it iterates over.
        * Thus, the iterator at hand contains references to references of integers.
        */
        .filter(|&&x| x <= pivot)
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

/*
* Uses the counting sort algorithm to sort the vector referenced by a. 
*
* Requires an upper bound s on the elements in the vector a.
* If there is a number above s in the vector a (invalid instance), counting sort returns an Err variant.
*
* Counting sort counts how often each number from {0, ..., s} occurs in the vector.
* It then creates a new vector into which it inserts all numbers as many times as they occured,
* starting with the lowest.
*/
pub fn counting_sort(a: &Vec<u32>, s: u32) -> Result<Vec<u32>, SortingInstanceError> {
    // check instance for validity: all values in a smaller/equal s?
    for &x in a.iter() { 
        if x > s { return Err(SortingInstanceError); }  
    }

    // create hash map number -> number of occurences (for numbers in {0, 1, ..., s})
    let mut counts = HashMap::<u32, u32>::new();

    // create empty result vector
    let mut result = vec![];

    // fill counts map
    for &x in a.iter() {
        let count = counts.entry(x).or_insert(0);
        *count += 1;
    }

    // fill result vector based on counts
    for i in 0..(s+1) { // range from 0 (inclusive) to s+1 (exclusive)
        for j in 0..(*(counts.entry(i).or_insert(0))) {
            result.push(i); // add j copies of i to the result vector where j is number of times i occurs in a
        }
    }

    Ok(result)
}

/*
* Struct modelling any error that could occur from a unsuitable input to a sorting algorithm.
*/
#[derive(Debug, PartialEq)]
pub struct SortingInstanceError;
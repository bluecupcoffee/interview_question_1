// # Question: Merge Ranges
//
// Write a Rust function that takes a list of ranges and merges all overlapping ranges. Each range is represented as an array with two elements: the start and end values.
//
//
// ### For example:
//
//
// #### Input:
//
//
// `[[1, 3], [2, 6], [8, 10], [15, 18]]`
//
//
// #### Output:
//
// `[[1, 6], [8, 10], [15, 18]]`
//
//
// #### Explanation:
//
//
// The ranges [1, 3] and [2, 6] overlap, so they are merged into [1, 6].
//
//
// #### Input:
//
//
// `[[1, 4], [4, 5]]`
//
//
// #### Output:
//
//
// `[[1, 5]]`
//
//
// #### Explanation:
//
//
// The ranges [1, 4] and [4, 5] touch at the end points and thus are merged into [1, 5].
//
//
// #### Input:
//
//
// `[[1, 5], [2, 3], [4, 8], [9, 10], [9, 12]]`
//
//
// #### Output:
//
//
// `[[1, 8], [9, 12]]`
//
//
// #### Explanation:
//
//
// The ranges [1, 5], [2, 3], and [4, 8] overlap and are merged into [1, 8]. The ranges [9, 10] and [9, 12] overlap and are merged into [9, 12].
//
//
// ### Requirements:
//
//
// 1. The function should sort the ranges first.
//
// 2. Explain your logic and each step of your solution.
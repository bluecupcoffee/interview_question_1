# Interview question
How would you write a Rust function that takes a list of ranges and merges all overlapping ranges?
## Constraints and expectations
- All ranges are 2-element i32 arrays composed of an upper bound and lower bound.
- There will be no negative integers
- It is possible that all 2-element arrays can overlap
- The arrays may be given out of order so they may need to be sorted.

## Examples
```
    let ranges = vec![[15, 18], [2, 6], [8, 10], [1, 3]];
    // expected output [1,6], [8,10], [15,18]

    let ranges2 = vec![[1, 4], [4, 5]];
    // expected output `[[1, 5]]`

    let ranges3 = vec![[1, 5], [2, 3], [4, 8], [9, 10], [9, 12]];
    // expected output `[[1, 8], [9, 12]]`

    let ranges4 = vec![[1,2], [1,3], [1,4], [1,5], [4,100]];
    // expected output [1,5], [10,100]
```
## Dev story
This most likely isn't the most elegant solution, but it is the completion of what I failed to complete during the interview. 
I ended up actually using a lot of what I had thrown out while my mind was just running through ideas as fast as possible. Prior to posting this solution, the unfinished solution had no broken check for starting a new range. It also did not have the 2-element temp array.

Granted, my unfinished solution *did* have these in some iterations, but I was rushing too fast to properly set down the logic. My issue was that my initial algorithm description was a bit too high in its level of description and then I tried to stick as close to it as possible. Now that I don't have the pressure of a test over me, I've refined it. You can see both at the bottom of the **merge_ranges** function in the main.rs file.

My goal isn't to provide the best solution as that would require other constraints and rules to satisfy. This is simply *a* solution. I was not able to show a working solution during an interview and I wanted to see it through.

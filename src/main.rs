mod question;

fn main() {

    // Define a vector of ranges

    let ranges = vec![[15, 18], [2, 6], [8, 10], [1, 3]];
    let ranges2 = vec![[1, 4], [4, 5]];
    let ranges3 = vec![[1, 5], [2, 3], [4, 8], [9, 10], [9, 12]];


    // Call merge_ranges function with the ranges

    let merged = merge_ranges(ranges);
    let merged2 = merge_ranges(ranges2);
    let merged3 = merge_ranges(ranges3);


    println!("Output: {:?}", merged);
    // expected output [1,6], [8,10], [15,18]

    println!("Output: {:?}", merged2);
    // expected output `[[1, 5]]`

    println!("Output: {:?}", merged3);
    // expected output `[[1, 8], [9, 12]]`
}

fn merge_ranges(mut v: Vec<[i32;2]>) -> Vec<[i32;2]> {

    //sort by first element
    v.sort_by(|a, b| a[0].cmp(&b[0]));

    // setup book keeping vars
    let mut v2: Vec<[i32;2]> = vec![];
    let mut broken: bool = true;
    let mut i = 0;
    let mut temp_arr: [i32;2] = [-1, -1];

    while i < v.len() {
        // establish new lower and upper bounds
        if broken {
            temp_arr[0] = v[i][0];
            temp_arr[1] = v[i][1];
        }
        if i + 1 < v.len() {
            if v[i+1][0] > temp_arr[1] {
                broken = true;
                v2.push(temp_arr.clone());

            } else {
                broken = false;
                if temp_arr[1] < v[i+1][1] {
                    temp_arr[1] = v[i+1][1];
                }
            }
        } else {
            v2.push(temp_arr.clone());
        }


        i = i + 1;


    }



    // 1. sort by first element of each array
    // 2. if array n[1] >= array n+1[0] start merging ranges
                // if n+1[1] >= n[1] -> range is [n[0], n+1[1]]
        // if not ranges are broken
            // simply start a new range
        //push new 2 element array

    v2
}
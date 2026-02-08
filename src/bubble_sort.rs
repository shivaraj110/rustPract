pub fn bubble_sort(arr: &mut [i32; 9]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

//Also bubble_sort but using while loops
pub fn bubble_sort_while(arr: &mut [i32; 9]) {
    let mut i = 0;
    while i < arr.len() {
        let mut j = 0;
        while j < arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}

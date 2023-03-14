// 泛型实现
fn bubble_sort_generics<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1);
            }
        }
    }
}

// 固定类型i32实现
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr_generics = vec!["5", "2", "9", "1", "8", "6"];
    println!("before bubble_sort_generics: {:?}", arr_generics);
    bubble_sort_generics(&mut arr_generics);
    assert_eq!(arr_generics, vec!["1", "2", "5", "6", "8", "9"]);
    println!("after bubble_sort_generics: {:?}", arr_generics); // 输出 ["1", "2", "5", "6", "8", "9"]

    let mut arr = vec![9, 1, 3, 2, 5, 6];
    println!("before bubble_sort: {:?}", arr);
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 5, 6, 9]);
    println!("after bubble_sort: {:?}", arr); // 输出 [1, 2, 3, 5, 6, 9]
}
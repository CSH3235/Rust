
fn main() {
    let mut arr = [2, 3, 1, 5, 4];

    println!("배열의 크기: {}", arr.len()); // arr.len() : 배열의 크기 출력


    let mut i = 0; // mut : 변수의 변형
    let mut min_index;
    let mut temp;
    let mut j;

    while i < arr.len() - 1 {
        min_index = i;
        j = i + 1;
        while j < arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }    
            j += 1;
        }
        temp = arr[min_index]; // 최솟값을 저장
        arr[min_index] = arr[i];
        arr[i] = temp; // 최솟값을 제일 앞으로 보냄
        i += 1;
    }

    i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }
}
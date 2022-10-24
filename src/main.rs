fn main() {
    println!("Hello, world!");

    let x:i32 = 5;

    // 要素数3の配列
    let mut array: [i32; 3] = [1, 2, 3];

    // 要素数3のベクタ
    let mut vec: Vec<i32> = vec![1, 2, 3];
    
    // if文
    if x == 5{
        println!("x=5");
    }

    // for文
    for i in 0..10 {
        println!("in for-loop: {}", i)
    }
}

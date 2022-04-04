// fn fizzbuzz(n: usize) {
//     for i in 0..n {
//         if i%15 == 0 {
//             println!("FizzBuzz");
//         } else if i%3 == 0 {
//             println!("Fizz");
//         } else if i%5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }

// fn square_sum(n: isize) -> isize {
//     (0..n) // 区間指定
//     .filter(|i| i%2 == 0) // iに対して偶数だけ抜き出す
//     .map(|i| i*i) // iに対してiを二乗
//     .sum() // 合計
// }

// fn square_sum(n: isize) -> isize {
//     (0..n).filter(|i| i%2 == 0).map(|i| i*i).sum()
// }

// fn rebind() {
//     let sum = 0;
//     for i in 0..10 {
//         let sum = sum + i;
//     }
//     println!("{}", sum);
// }

// fn reassign() {
//     let mut sum = 0;
//     for i in 0..10 {
//         sum = sum + i;
//     }
//     println!("{}", sum);
// }


fn main() {
    // fizzbuzz(0);
    // println!("{}", square_sum(10));
    // rebind();
    // reassign();

    // let x = 1;
    // let y: isize = x;
    // println!("{}", y);

    // let mut a = 1;
    // let b = &mut a;
    println!("Hello");
}

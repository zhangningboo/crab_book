fn main() {

    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Chordata", "Vertebrata", "Mammalia", "Carnivora", "Felidae"];
    assert_eq!(lazy_caterer[0], 1);
    assert_eq!(taxonomy.len(), 6);

    let mut sieve = [true; 1000];
    for i in 2..1000 {
        if sieve[i] {
            let mut j = i * i;
            while j < 1000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let mut init_cache_block = [0u8; 1024];  // 1kb的缓冲区
    init_cache_block[1000] = 12;
    assert_eq!(init_cache_block.len(), 1024);

    let primes = vec![1, 2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, ["panama", "a canal", "a plan", "a man"]);

    println!("{}", palindrome[0..2].join("_"));  // 类似python的切片

    let ori_str = r"D:\Downloads\a.txt";  // 原始字符串，类似python的rf
    println!("{}", ori_str);
    println!(r###"Line 1
    Line2 r###
       Line3 'r###' "r###"
    "###);  // 行前的空格也会输出
    println!(r###"Line 1
Line2
Line3
    "###);  // 行前无空格顶头输出

    let byte_str = b"GET";  // 字节串
    assert_eq!(byte_str, &[b'G', b'E', b'T']);

    assert_eq!("❁_❁".len(), 7);  // .len()返回的是字节数
    assert_eq!("❁_❁".chars().count(), 3);
}

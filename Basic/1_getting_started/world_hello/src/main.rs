fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    // Rust 数组不能和Python一样进入循环，需要使用 .iter
    for region in regions.iter() {
        println!("{}", &region);
        // ! 是宏操作符
        // Rust 会自动识别输出数据类型，因此可以直接使用 {}
    }
}

fn main() {
    greet_world();
}




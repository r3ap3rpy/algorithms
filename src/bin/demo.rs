use algorithmz::sorting::stooge_sort;

fn main() {
    let my_list = vec![3, 5, 2, 1, 6, 4];
    match stooge_sort(&my_list) {
        Ok(n) => println!("The result was: {:?}",n),
        Err(e) => eprintln!("The error was: {}",e),
    }
    //let result = algorithmz::sorting::merge_sort(&[2,1,3,5,4,7,6,8]).unwrap();
    //assert_eq!(result,[1,2,3,4,5,6,7,8]);
}

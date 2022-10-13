fn f_split_box(x: usize) -> Box<dyn Fn(usize) -> usize> {
    if true {
        Box::new(move |y: usize| x + y)
    } else {
        Box::new(move |y: usize| x - y)
    }
}

fn f(x: usize) -> impl Fn(usize) -> usize {
    move |y: usize| x + y
}

fn g<F>(f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    f(5)
}

fn main() {
    println!("{}", g(f(1)));
}

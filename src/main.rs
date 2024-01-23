mod api;
mod overview;
mod thread;

fn main() {
    let a = api::get_threads(1).unwrap();
    println!("{:?}", a);

    let b = api::get_thread(&a[0]).unwrap();
    println!("{:?}", b);
    println!("\n\n\n{}", b.comments.len());
}

use lunatic::{spawn_link, Mailbox};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    // [0..1000].iter().for_each(|_| {
    //     spawn_link!(@task || {
    //         println!("Hi! I'm a process.");
    //     });
    // });
    let child = spawn_link!(@task || {
        // This closure gets a new heap and stack to
        // execute on, and can't access the memory of
        // the parent process.
        println!("Hi! I'm a process.");
    });
    // Wait for child to finish
    let _ = child.result();
}

#[lunatic::test]
fn it_works() {
    println!("Lunatic test");
}

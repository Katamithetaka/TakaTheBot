mod slash_commands;
fn main() {
    println!("{:?}", slash_commands::get_commands().keys());
}

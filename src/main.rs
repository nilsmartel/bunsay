fn main() {
    let bun = r"
(\_/)
( ._.)
/ >"
    .trim();

    let say = std::env::args().skip(1).fold(String::new(), |mut s, e| {
        s.push(' ');
        s.push_str(&e);
        s
    });

    println!("{bun}{say}");
}

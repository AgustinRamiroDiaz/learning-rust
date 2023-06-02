fn main() {
    let m1 = Mufasa("outer");
    {
        let m2 = Mufasa("inner");
    }
}

struct Mufasa(&'static str);

impl Drop for Mufasa {
    fn drop(&mut self) {
        println!("Long live the king ({})", self.0)
    }
}

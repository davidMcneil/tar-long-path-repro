use tar::Archive;
use std::fs::File;

fn main() {
    let file = File::open("test/test.tar").expect("failed to open tar file");
    let mut tar = Archive::new(file);
    tar.unpack("test/expanded/").expect("failed to unpack tar file");
}

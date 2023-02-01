fn main() {
    {
        let s = "hello";
        println!("s: {:?}, {:?}", s, s.as_ptr());

        let s1: &str = s.as_ref();
        println!("s1: {:?}, {:?}", s1, s1.as_ptr());

        // turbofish
        let s2 = AsRef::<str>::as_ref(s);
        println!("s2: {:?}, {:?}", s2, s2.as_ptr());

        let s3: &[u8] = s.as_ref();
        println!("s3: {:?}, {:?}", s3, s3.as_ptr());

        // turbofish
        let s4 = AsRef::<[u8]>::as_ref(s);
        println!("s4: {:?}, {:?}", s4, s4.as_ptr());

        let s5 = &s;
        println!("s5: {:?}, {:?}", s5, s5.as_ptr());
    }
}

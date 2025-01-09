fn main() {
    #[cfg(feature = "monospace-font")]
    {
        let mut f = text2v::monospace_font();
        let path_iter = f.render("Hello, World!", 200.0);
        for path_el in path_iter.0 {
            println!("{:?}", path_el);
        }
    }
}

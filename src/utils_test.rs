mod test{
    use crate::utils::get_attrs;
    #[test]
    fn test_get_attrs() {
        let path = "/mnt/d/Code/rust/spatial-import-server/data/beijing.shp";
        get_attrs(path);
        println!("111111");
    }
}

    

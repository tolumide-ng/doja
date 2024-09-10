#[cfg(test)]
mod uci_tests {
    use std::io::Cursor;

    use crate::uci::UCI;

    #[test]
    fn should_return_author_identity() {
        let identity = UCI::identify();
        assert_eq!(identity[0], "id name: papa");
        assert_eq!(identity[1], "id author: Tolumide");
        assert_eq!(identity[2], "id email: tolumideshopein@gmail.com");
        assert_eq!(identity[3], "uciok");
    }

    #[test]
    fn should_respond_to_the_uciok_cmd() {
        // let mut buffer = vec![0; 30];
        let mut cursor = Cursor::new(vec![0; 40]);
        
        let _ = UCI::default().process_input(String::from("uciok"), cursor);
        let xx = cursor.get_ref()[..];
        // println!("the cursosr is {:#?}", )
    }
}
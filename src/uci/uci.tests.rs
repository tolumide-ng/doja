#[cfg(test)]
mod uci_tests {
    use crate::uci::UCI;

    #[test]
    fn should_return_author_identity() {
        let identity = UCI::identify();
        assert_eq!(identity[0], "id name: papa");
        assert_eq!(identity[1], "id author: Tolumide");
        assert_eq!(identity[2], "id email: tolumideshopein@gmail.com");
    }
}
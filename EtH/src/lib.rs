extern crate crypto;
extern crate sha1;
mod state;
mod hash;
mod otae;
mod sarcad;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

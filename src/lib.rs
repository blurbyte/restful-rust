mod filters;
pub mod routes;
mod schema;
mod validators;

#[cfg(test)]
mod tests {
    #[test]
    fn shall_pass() {
        assert_eq!(2 + 1, 3);
    }
}

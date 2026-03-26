use chimitheque_utils::casbin::build_casbin_matchers;

#[cfg(not(tarpaulin_include))]
fn main() {
    build_casbin_matchers();
}

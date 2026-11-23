//! Prints the casbin matcher of the permissions, to copy into the
//! `[matchers]` section of `chimitheque_back/src/casbin/policy.conf`.
//!
//! Run it with `cargo run --example casbin`. Make changes in `casbin.xlsx`
//! too.

fn main() {
    // Request person must match policy person.
    let person_request_match = "(r.person_id == p.person_id)";
    // Admin.
    let is_admin_match = r#"(p.perm == "all" && p.item == "all" && p.entity_id == "")"#;
    // The request action, from the HTTP method: r (GET), c (POST), u (PUT) or
    // d (DELETE). A read is given by a r, w or all permission; a create, update
    // or delete by a w or all permission.
    let permission_equivalence_match = r#"\
        (\
        (r.action == "r" && (p.perm == "r" || p.perm == "w" || p.perm == "all")) || \
        ((r.action == "c" || r.action == "d" || r.action == "u") && (p.perm == "w" || p.perm == "all")) \
        )"#;
    // Permissions definition.
    // Toggling a borrowing (GET /borrows/{storage id}) needs to read the storages
    // of the entity, on purpose: the b (borrow) permission is not used yet.
    // A manager can only be updated by an admin.
    let rules_match = r#"\
    ( (r.item == "products" && r.action == "c")                    && (p.item == "products" || p.item =="all") ) || \
    ( (r.item == "products" && r.action == "r" && r.item_id == "") && (p.item == "products" || p.item =="all") ) || \
    ( (r.item == "products" && r.action == "r" && r.item_id != "") && (p.item == "products" || p.item =="all") ) || \
    ( (r.item == "products" && r.action == "u")                    && (p.item == "products" || p.item =="all") ) || \
    ( (r.item == "products" && r.action == "d")                    && (p.item == "products" || p.item =="all") && !matchProductHasStorages(r.item_id) ) || \
    \
    ( (r.item == "storages" && r.action == "c")                    && (p.item == "storages" || p.item =="all") ) || \
    ( (r.item == "storages" && r.action == "r" && r.item_id == "") && (p.item == "storages" || p.item =="all") ) || \
    ( ((r.item == "storages" || r.item == "borrows") && r.action == "r" && r.item_id != "") && (p.item == "storages" || p.item =="all") && (p.entity_id == "" || matchStorageIsInEntity(r.item_id,p.entity_id)) ) || \
    ( (r.item == "storages" && r.action == "u")                    && (p.item == "storages" || p.item =="all") && (p.entity_id == "" || matchStorageIsInEntity(r.item_id,p.entity_id)) ) || \
    ( (r.item == "storages" && r.action == "d")                    && (p.item == "storages" || p.item =="all") && (p.entity_id == "" ||matchStorageIsInEntity(r.item_id,p.entity_id)) ) || \
    \
    ( (r.item == "store_locations" && r.action == "c")                    && (p.item == "entities" || p.item =="all") ) || \
    ( (r.item == "store_locations" && r.action == "r" && r.item_id == "") && (p.item == "storages" || p.item =="all") ) || \
    ( (r.item == "store_locations" && r.action == "r" && r.item_id != "") && (p.item == "storages" || p.item =="all") && (p.entity_id == "" || matchStoreLocationIsInEntity(r.item_id,p.entity_id)) ) || \
    ( (r.item == "store_locations" && r.action == "u")                    && (p.item == "entities" || p.item =="all") && (p.entity_id == "" || matchStoreLocationIsInEntity(r.item_id,p.entity_id)) ) || \
    ( (r.item == "store_locations" && r.action == "d")                    && (p.item == "entities" || p.item =="all") && (p.entity_id == "" || matchStoreLocationIsInEntity(r.item_id,p.entity_id)) && !matchStoreLocationHasChildren(r.item_id) && !matchStoreLocationHasStorages(r.item_id) ) || \
    \
    ( (r.item == "people" && r.action == "c")                    && (p.item == "entities" || p.item =="all") ) || \
    ( (r.item == "people" && r.action == "r" && r.item_id == "") && (p.item == "entities" || p.item =="all") ) || \
    ( (r.item == "people" && r.action == "r" && r.item_id != "") && (p.item == "entities" || p.item =="all") && (p.entity_id == "" || matchPersonIsInEntity(r.item_id,p.entity_id)) ) || \
    ( (r.item == "people" && r.action == "u")                    && (r.person_id != r.item_id) && (p.item == "entities" || p.item == "all") && (p.entity_id == "" || matchPersonIsInEntity(r.item_id,p.entity_id)) && !matchPersonIsAdmin(r.item_id) && (matchPersonIsAdmin(p.person_id) || !matchPersonIsManager(r.item_id)) ) || \
    ( (r.item == "people" && r.action == "d")                    && (r.person_id != r.item_id) && (matchPersonIsAdmin(p.person_id)) && !matchPersonIsManager(r.item_id) && !matchPersonIsAdmin(r.item_id) ) || \
    \
    ( (r.item == "entities" && r.action == "c")                    && (matchPersonIsAdmin(p.person_id)) ) || \
    ( (r.item == "entities" && r.action == "r" && r.item_id == "") && (p.item == "entities" || p.item =="all") ) || \
    ( (r.item == "entities" && r.action == "r" && r.item_id != "") && (r.item_id == p.entity_id || p.entity_id == "") && (p.item == "entities" || p.item =="all") ) || \
    ( (r.item == "entities" && r.action == "u")                    && (matchPersonIsAdmin(p.person_id)) ) || \
    ( (r.item == "entities" && r.action == "d")                    && (matchPersonIsAdmin(p.person_id)) && !matchEntityHasMembers(r.item_id) && !matchEntityHasStoreLocations(r.item_id) ) || \
    \
    ( (r.item == "bookmarks") && (p.item == "products" || p.item =="all") ) || \
    \
    ( (r.item == "stocks") && (p.item == "storages" || p.item =="all") )"#;
    println!(
        "m = ( {person_request_match} && ( {is_admin_match} || {permission_equivalence_match} ) && ( {rules_match} ) )"
    );
    // println!("m = ( product1_match || product2_match )");
}

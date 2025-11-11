// Make changes in casbin.xlsx too.
pub fn build_casbin_matchers() {
    // Request person must match policy person.
    let person_request_match = r#"(r.person_id == p.person_id)"#;
    // Admin.
    let is_admin_match = r#"(p.perm == "all" && p.item == "all" && p.entity_id == "-1")"#;
    // The policy action match the request action
    // or if the action is read the policy can be r or w or all
    // or if the action is write or delete the policy can be w or all
    // or if the action is all the policy must be all (redondant with the first sentence but we keep it for readability)
    let permission_equivalence_match = r#"(r.action == p.perm || (r.action == "r" && (p.perm == "w" || p.perm == "all")) || ((r.action == "w" || r.action == "d") && p.perm == "all") || (r.action == "all" && p.perm == "all"))"#;
    // Permissions definition.
    let rules_match = r#"\
    ((r.item == "products" && r.action == "c") && p.perm == "w" && (p.item == "products" || p.item =="all")) || \
    ((r.item == "products" && r.action == "r" && r.item_id == "") && p.perm == "r" && (p.item == "products" || p.item =="all")) || \
    ((r.item == "products" && r.action == "r" && r.item_id != "") && p.perm == "r" && (p.item == "products" || p.item =="all")) || \
    ((r.item == "products" && r.action == "u") && p.perm == "w" && (p.item == "products" || p.item =="all")) || \
    ((r.item == "products" && r.action == "d") && p.perm == "w" && (p.item == "products" || p.item =="all")) || \
    ((r.item == "storages" && r.action == "c") && p.perm == "w" && (p.item == "storages" || p.item =="all")) || \
    ((r.item == "storages" && r.action == "r" && r.item_id == "") && p.perm == "r" && (p.item == "storages" || p.item =="all")) || \
    ((r.item == "storages" && r.action == "r" && r.item_id != "") && p.perm == "r" && (p.item == "storages" || p.item =="all") && matchPersonAndStorageAreInEntity({r.person_id},{r.item_id},{p.entity_id})) || \
    ((r.item == "storages" && r.action == "u") && p.perm == "w" && (p.item == "storages" || p.item =="all") && matchPersonAndStorageAreInEntity({r.person_id},{r.item_id},{p.entity_id})) || \
    ((r.item == "storages" && r.action == "d") && p.perm == "w" && (p.item == "storages" || p.item =="all") && matchPersonAndStorageAreInEntity({r.person_id},{r.item_id},{p.entity_id})) || \
    ((r.item == "store_locations" && r.action == "c") && p.perm == "w" && (p.item == "entities" || p.item =="all")) || \
    ((r.item == "store_locations" && r.action == "r" && r.item_id == "") && p.perm == "r" && (p.item == "entities" || p.item =="all")) || \
    ((r.item == "store_locations" && r.action == "r" && r.item_id != "") && p.perm == "r" && (p.item == "entities" || p.item =="all") && matchPersonAndStoreLocationAreInEntity({r.person_id},{r.item_id},{p.entity_id})) || \
    ((r.item == "store_locations" && r.action == "u") && p.perm == "w" && (p.item == "entities" || p.item =="all") && matchPersonAndStoreLocationAreInEntity({r.person_id},{r.item_id},{p.entity_id})) || \
    ((r.item == "store_locations" && r.action == "d") && p.perm == "w" && (p.item == "entities" || p.item =="all") && matchPersonAndStoreLocationAreInEntity({r.person_id},{r.item_id},{p.entity_id}) && !matchStoreLocationHasChildren({r.item_id}) && !matchStoreLocationHasStorages({r.item_id})) || \
    ((r.item == "people" && r.action == "c") && p.perm == "w" && (p.item == "entities" || p.item =="all")) || \
    ((r.item == "people" && r.action == "r" && r.item_id == "") && p.perm == "r" && (p.item == "entities" || p.item =="all")) || \
    ((r.item == "people" && r.action == "r" && r.item_id != "") && p.perm == "r" && (p.item == "entities" || p.item =="all") && matchPersonAndPersonAreInEntity({p.entity_id},{r.item_id})) || \
    ((r.item == "people" && r.action == "u") && p.perm == "w" && (p.item == "entities" || p.item =="all") && matchPersonAndPersonAreInEntity({p.entity_id},{r.item_id})) || \
    ((r.item == "people" && r.action == "d") && p.perm == "w" && (p.item == "entities" || p.item =="all") && matchPersonAndPersonAreInEntity({p.entity_id},{r.item_id}) && !matchPersonIsManager({r.item_id})&& !matchPersonIsadmin({r.item_id})) || \
    ((r.item == "entities" && r.action == "c") && p.perm == "w" && p.entity_id == -1 && p.item == all) || \
    ((r.item == "entities" && r.action == "r" && r.item_id == "") && p.perm == "r") || \
    ((r.item == "entities" && r.action == "r" && r.item_id != "") && p.perm == "r" && r.item_id == p.entity_id) || \
    ((r.item == "entities" && r.action == "u") && p.perm == "w" && r.item_id == p.entity_id) || \
    ((r.item == "entities" && r.action == "d") && p.perm == "w" && p.entity_id == -1 && p.item == all && !matchEntityHasMembers({r.item_id}) && !matchEntityHasStoreLocations({r.item_id})) || \
    ((r.item == bookmarks" || r.item == "download" || r.item == "validate") && (p.item == "products" || p.item =="all")) || \
    ((r.item == userinfo" || r.item == "ping"))"#;

    println!(
        "m = ( {person_request_match} && ( {is_admin_match} || {permission_equivalence_match} ) && ( {rules_match} ) )"
    );
    // println!("m = ( {product1_match} || {product2_match} )");
}

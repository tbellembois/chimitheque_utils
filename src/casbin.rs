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
    // Action on bookmars or download or validate requires only permission on products (r,w,d).
    let bookmark_match = r#"((r.item == "bookmarks" || r.item == "download" || r.item == "validate") && (p.item == "products" || p.item =="all"))"#;
    // Can only delete products without storages.
    let product1_match =
        r#"(r.item == "products" && r.action == "d" && ! matchProductHasStorages(r.item_id))"#;
    // Read or write products requires the permission on products or all.
    let product2_match = r#"(r.item == "products" && (r.action == "r" || r.action == "w") && (p.item == "products" || p.item =="all"))"#;
    // Request to read any (at least one) entity requires policy for at least one entity or all items.
    // (actually everybody has this permission given that each user can read its own entity)
    let entity1_match = r#"(r.item == "entities" && r.action == "r" && r.item_id == "" && (p.item == "entities" || p.item =="all"))"#;
    // Request to read a specific entity requires to be member of the entity.
    let entity2_match = r#"(r.item == "entities" && r.action == "r" && r.item_id != "" && matchPersonIsInEntity(r.person_id, r.item_id))"#;
    // Request to create an entity only for admins
    let entity3_match = r#"(r.item == "entities" && r.action == "w" && r.item_id == "" && (p.perm == "all" && p.item == "all" && p.entity_id == "-1"))"#;
    // Request to delete a specitic entity requires to have policy to write all entites (admin) - and entity must have no members and no store locations.
    let entity4_match = r#"(r.item == "entities" && r.action == "d" && r.item_id != "" && (p.perm == "all" && p.item == "all" && p.entity_id == "-1") && ! matchEntityHasMembers(r.item_id) && ! matchEntityHasStoreLocations(r.item_id))"#;
    // Request to write a specitic entity requires to have policy to write the entity or all.
    let entity5_match = r#"(r.item == "entities" && r.action == "w" && r.item_id == p.entity_id && (p.item == "entities" || p.item =="all") && matchPersonIsInEntity(r.person_id, r.item_id))"#;
    // Request to read/write/delete any (at least one) storage requires policy for at least one storage or all.
    let storage1_match = r#"((r.item == "storages" || r.item == "stocks" || r.item == "borrows") && r.item_id == "" && (p.item == "storages" || p.item =="all"))"#;
    // Request to read/write/delete a specific storage requires to be member of the same entity as the storage store location.
    let storage2_match = r#"((r.item == "storages" || r.item == "stocks" || r.item == "borrows") && r.item_id != "" && matchPersonIsInStorageEntity(r.person_id, r.item_id, p.entity_id))"#;
    // Request to read any (at least one) store location requires policy of at least one storage or all.
    let store_location1_match = r#"(r.item == "store_locations" && r.action == "r" && r.item_id == "" && (p.item == "storages" || p.item =="all"))"#;
    // Request to read a specific store location requires policy for storage and to be member of the same entity as the store location
    let store_location2_match = r#"(r.item == "store_locations" && r.action == "r" && r.item_id != "" && matchPersonIsInStoreLocationEntity(r.person_id, r.item_id, p.entity_id))"#;
    // Request to delete any (at least one) store location requires policy of at least one entity or all - and store location must have no children and storages.
    let store_location3_match = r#"(r.item == "store_locations" && r.action == "d" && r.item_id != "" && (p.item == "entities" || p.item =="all") && ! matchStoreLocationHasChildren(r.item_id) && ! matchStoreLocationHasStorages(r.item_id))"#;
    // Request to write any (at least one) store location requires policy of at least one entity or all.
    let store_location4_match = r#"(r.item == "store_locations" && (r.action == "w" || r.action == "d") && r.item_id == "" && (p.item == "entities" || p.item =="all"))"#;
    // Request to write/delete a specific store location requires policy for entity and to be member of the same entity as the store location.
    let store_location5_match = r#"(r.item == "store_locations" && (r.action == "w" || r.action == "d") && r.item_id != "" && matchPersonIsInStoreLocationEntity(r.person_id, r.item_id, p.entity_id))"#;
    // Request to read any (at least one) person requires policy of at least one entity or all.
    let people1_match = r#"(r.item == "people" && r.action == "r" && r.item_id == "" && (p.item == "entities" || p.item =="all"))"#;
    // Request to read a specific person (except oneself) requires policy for entity and to be member of the same entity as the person.
    let people2_match = r#"(r.item == "people" && r.action == "r" && r.item_id != p.person_id && r.item_id != "" && matchPersonIsInPersonEntity(r.person_id, r.item_id, p.entity_id))"#;
    // Can not delete oneself oan admin or a manager.
    let people3_match = r#"(r.item == "people" && r.action == "d" && r.item_id != "" && (p.item == "entities" || p.item =="all") && r.item_id != p.person_id && ! matchPersonIsAdmin(r.person_id) && ! matchPersonIsManager(r.person_id))"#;
    // Request to write/delete any (at least one) person requires policy of at least one entity or all.
    let people4_match = r#"(r.item == "people" && (r.action == "w" || r.action == "d") && r.item_id == "" && (p.item == "entities" || p.item =="all"))"#;
    // -> Request to write/delete a specific person (except oneself) requires policy for entity and to be member of the same entity as the person
    let people5_match = r#"(r.item == "people" && (r.action == "w" || r.action == "d") && r.item_id != p.person_id && r.item_id != "" && matchPersonIsInPersonEntity(r.person_id, r.item_id))"#;
    // No restriction to retrieve user information.
    let userinfo_match = r#"(r.item == "userinfo")"#;
    // No restriction to ping the server.
    let ping_match = r#"(r.item == "ping")"#;

    let match_items = format!("( {bookmark_match} || {product1_match} || {product2_match} || {entity1_match} || {entity2_match} || {entity3_match} || {entity4_match} || {entity5_match} || {storage1_match} || {storage2_match} || {store_location1_match} || {store_location2_match} || {store_location3_match} || {store_location4_match} || {store_location5_match} ||  {people1_match} || {people2_match} || {people3_match} || {people4_match} || {people5_match} )");

    println!(
        "m = ( {person_request_match} && ( {is_admin_match} || {permission_equivalence_match} ) && ( {userinfo_match} || {ping_match} || {match_items} ) )"
    );
    // println!("m = ( {product1_match} || {product2_match} )");
}

use bin_packer_3d::packing_algorithm::{packing_algorithm};
use bin_packer_3d::item::{Item};

#[test]
fn test_pack_items_one_item() {
    let bin = [3.0, 4.5, 5.0];
    let items = vec![Item::new(String::from("item1"), [3.0, 4.5, 5.0])];
    let res = packing_algorithm(bin, &items);
    assert_eq!(res, vec![vec![String::from("item1")]]);
}


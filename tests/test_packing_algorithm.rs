use bin_packer_3d::bin::Bin;
use bin_packer_3d::error::{Error, Result};
use bin_packer_3d::item::Item;
use bin_packer_3d::packing_algorithm::packing_algorithm;

#[test]
fn test_pack_items_no_fit() -> Result<()> {
    let items = vec![Item::new("item1", [3.0, 4.5, 6.0])];
    let err = packing_algorithm(Bin::new([3.0, 4.5, 5.0]), &items).unwrap_err();
    assert_eq!(
        err,
        Error::ItemsNoFit(format!("All items must fit within the bin dimensions."))
    );
    Ok(())
}

#[test]
fn test_pack_items_one_item() -> Result<()> {
    let items = vec![Item::new("item1", [3.0, 4.5, 5.0])];
    let res = packing_algorithm(Bin::new([3.0, 4.5, 5.0]), &items)?;
    assert_eq!(res, vec![vec!["item1"]]);
    Ok(())
}

#[test]
fn test_pack_items_two_item_exact() -> Result<()> {
    let bin = Bin::new([13.0, 26.0, 31.0]);
    let item_1 = Item::new("item1", [13.0, 13.0, 31.0]);
    let items = vec![item_1.clone(), item_1];
    let res = packing_algorithm(bin, &items)?;
    assert_eq!(
        res,
        vec![vec!["item1", "item1"]]
    );
    Ok(())
}


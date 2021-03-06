use bin_packer_3d::bin::Bin;
use bin_packer_3d::error::{Error, Result};
use bin_packer_3d::item::{Item, ItemId};
use bin_packer_3d::packing_algorithm::packing_algorithm;

/// test packing_algorithm API

#[test]
fn test_pack_items_no_items() -> Result<()> {
    let items = vec![];
    let res = packing_algorithm(Bin::new([3, 4, 5]), &items)?;
    assert_eq!(res, Vec::<Vec<&ItemId>>::new());
    Ok(())
}

#[test]
fn test_pack_items_no_fit() -> Result<()> {
    let items = vec![Item::new("item1", [3, 4, 6])];
    let err = packing_algorithm(Bin::new([3, 4, 5]), &items).unwrap_err();
    assert_eq!(
        err,
        Error::AllItemsMustFit(format!("All items must fit within the bin dimensions."))
    );
    Ok(())
}

#[test]
fn test_pack_items_one_item() -> Result<()> {
    let items = vec![Item::new("item1", [3, 4, 5])];
    let res = packing_algorithm(Bin::new([3, 4, 5]), &items)?;
    assert_eq!(res, vec![vec!["item1"]]);
    Ok(())
}

#[test]
fn test_pack_items_two_item_exact() -> Result<()> {
    let bin = Bin::new([13, 26, 31]);
    let item_1 = Item::new("item1", [13, 13, 31]);
    let items = vec![item_1, item_1];
    let res = packing_algorithm(bin, &items)?;
    assert_eq!(res, vec![vec!["item1", "item1"]]);
    Ok(())
}

#[test]
fn test_two_items_two_bins() -> Result<()> {
    let item = Item::new("item1", [13, 13, 31]);
    let items = vec![item, item];
    let res = packing_algorithm(Bin::new([13, 13, 31]), &items)?;
    assert_eq!(res, vec![vec!["item1"], vec!["item1"]]);
    Ok(())
}

#[test]
fn test_three_items_one_bin() -> Result<()> {
    let item_1 = Item::new("item1", [13, 13, 31]);
    let item_2 = Item::new("item2", [8, 13, 31]);
    let item_3 = Item::new("item3", [5, 13, 31]);
    let items = vec![item_1, item_2, item_3];
    let res = packing_algorithm(Bin::new([13, 26, 31]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_one_overflow() -> Result<()> {
    let item = Item::new("item1", [1, 1, 1]);
    let items = [item; 28];
    let res = packing_algorithm(Bin::new([3, 3, 3]), &items)?;
    assert_eq!(res, vec![["item1"; 27].to_vec(), vec!["item1"]]);
    Ok(())
}

#[test]
fn test_odd_sizes() -> Result<()> {
    let item_1 = Item::new("item1", [3, 8, 10]);
    let item_2 = Item::new("item2", [1, 2, 5]);
    let item_3 = Item::new("item3", [1, 2, 2]);
    let items = vec![item_1, item_2, item_2, item_3];
    let res = packing_algorithm(Bin::new([10, 20, 20]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_odd_sizes_unordered() -> Result<()> {
    // test odd sized items will be sorted to fit.
    let item_1 = Item::new("item1", [3, 8, 10]);
    let item_2 = Item::new("item2", [1, 2, 5]);
    let item_3 = Item::new("item3", [1, 2, 2]);
    let items = vec![item_3, item_2, item_1, item_2];
    let res = packing_algorithm(Bin::new([10, 20, 20]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_slightly_larger_bin() -> Result<()> {
    let item = Item::new("item1", [4, 4, 12]);
    let items = vec![item, item];
    // let res = packing_algorithm(Bin::new([5, 8, 12]), &items)?;
    let res = packing_algorithm(Bin::new([4, 8, 12]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item1"]]);
    Ok(())
}

#[test]
fn test_pack_3_bins() -> Result<()> {
    let item = Item::new("item1", [4, 4, 12]);
    let items = vec![item, item, item];
    let res = packing_algorithm(Bin::new([4, 4, 12]), &items)?;
    assert_eq!(res, vec![vec!["item1"], vec!["item1"], vec!["item1"]]);
    Ok(())
}

#[test]
fn test_dim_over_2() -> Result<()> {
    // test that when length of item <= length of bin / 2 it packs along longer # edge

    let item = Item::new("item1", [3, 4, 5]);
    let items = [item; 4];
    let res = packing_algorithm(Bin::new([6, 8, 10]), &items)?;
    assert_eq!(res, vec![["item1"; 4].to_vec()]);
    Ok(())
}

#[test]
fn test_odd_sizes_again() -> Result<()> {
    // test items with different dimensions will be rotated to fit into one bin

    let item_1 = Item::new("item1", [1, 18, 19]);
    let item_2 = Item::new("item2", [17, 18, 18]);
    let item_3 = Item::new("item3", [1, 17, 18]);
    let items = vec![item_1, item_2, item_3];
    let res = packing_algorithm(Bin::new([18, 18, 19]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_100_items_inexact_fit() -> Result<()> {
    // test many items into one bin with inexact fit

    let item = Item::new("item1", [5, 5, 5]);
    let items = [item; 100];
    let res = packing_algorithm(Bin::new([51, 51, 6]), &items)?;
    assert_eq!(res.len(), 1);
    Ok(())
}

#[test]
fn test_100_items_inexact_fit_2_bins() -> Result<()> {
    // test many items separated into 2 bins with exact fit

    let item = Item::new("item1", [5, 5, 5]);
    let items = [item; 100];
    let res = packing_algorithm(Bin::new([25, 10, 25]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res.first().map(|packed| packed.len()), Some(50));
    assert_eq!(res.last().map(|packed| packed.len()), Some(50));
    Ok(())
}

#[test]
fn test_big_die_and_serveral_decks_of_cards() -> Result<()> {
    let deck = Item::new("deck", [2, 8, 12]);
    let die = Item::new("die", [8, 8, 8]);
    let items = vec![deck, deck, die, deck, deck];
    let res = packing_algorithm(Bin::new([8, 8, 12]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res, vec![["deck"; 4].to_vec(), vec!["die"]]);
    Ok(())
}

#[test]
fn test_tight_fit_many_oblong() -> Result<()> {
    // tests a tight fit for non-cubic items

    let item = Item::new("item1", [1, 2, 3]);
    let items = [item; 107];
    let res = packing_algorithm(Bin::new([8, 9, 9]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res, vec![["item1"; 106].to_vec(), vec!["item1"]]);
    Ok(())
}

#[test]
fn test_tight_fit_many_oblong_inexact() -> Result<()> {
    // tests that the algorithm remains at least as accurate as it already is. If it were perfect,
    // the first bin would have 48 in it
    let item = Item::new("item1", [1, 2, 3]);
    let items = [item; 49];
    let res = packing_algorithm(Bin::new([4, 8, 9]), &items)?;
    assert_eq!(res.len(), 2);
    assert!(res.first().map(|packed| packed.len()) >= Some(44));
    Ok(())
}

#[test]
fn test_flat_bin() -> Result<()> {
    let item_1 = Item::new("item1", [1.25, 7.0, 10.0]);
    let items = vec![item_1, item_1, item_1];
    let res = packing_algorithm(Bin::new([3.5, 9.5, 12.5]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res.first().map(|packed| packed.len()), Some(2));
    Ok(())
}

/// Test Bin API

// NOTE: It's probably worth re-organizing our integration tests, perhaps grouping them by module.

#[test]
fn test_bin_try_packing() -> Result<()> {
    let item_1 = Item::new("item1", [24, 10, 2]);
    let item_2 = Item::new("item2", [24, 10, 2]);
    let item_3 = Item::new("item3", [24, 10, 2]);
    let mut bin = Bin::new([24, 10, 4]);
    assert!(bin.try_packing(item_1).is_some());
    assert!(bin.try_packing(item_2).is_some());
    assert_eq!(bin.try_packing(item_3), None);
    assert_eq!(
        bin.items
            .into_iter()
            .map(|item| item.id)
            .collect::<Vec<&ItemId>>(),
        vec!["item1", "item2"]
    );
    Ok(())
}

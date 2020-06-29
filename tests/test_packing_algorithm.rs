use bin_packer_3d::bin::Bin;
use bin_packer_3d::error::{Error, Result};
use bin_packer_3d::item::{Item, ItemId};
use bin_packer_3d::packing_algorithm::packing_algorithm;

#[test]
fn test_pack_items_no_items() -> Result<()> {
    let items = vec![];
    let res = packing_algorithm(Bin::new([3.0, 4.5, 5.0]), &items)?;
    assert_eq!(res, Vec::<Vec<&ItemId>>::new());
    Ok(())
}

#[test]
fn test_pack_items_no_fit() -> Result<()> {
    let items = vec![Item::new("item1", [3.0, 4.5, 6.0])];
    let err = packing_algorithm(Bin::new([3.0, 4.5, 5.0]), &items).unwrap_err();
    assert_eq!(
        err,
        Error::AllItemsMustFit(format!("All items must fit within the bin dimensions."))
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
    assert_eq!(res, vec![vec!["item1", "item1"]]);
    Ok(())
}

#[test]
fn test_two_items_two_bins() -> Result<()> {
    let item = Item::new("item1", [13.0, 13.0, 31.0]);
    let items = vec![item.clone(), item];
    let res = packing_algorithm(Bin::new([13.0, 13.0, 31.0]), &items)?;
    assert_eq!(res, vec![vec!["item1"], vec!["item1"]]);
    Ok(())
}

#[test]
fn test_three_items_one_bin() -> Result<()> {
    let item_1 = Item::new("item1", [13.0, 13.0, 31.0]);
    let item_2 = Item::new("item2", [8.0, 13.0, 31.0]);
    let item_3 = Item::new("item3", [5.0, 13.0, 31.0]);
    let items = vec![item_1, item_2, item_3];
    let res = packing_algorithm(Bin::new([13.0, 26.0, 31.0]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_one_overflow() -> Result<()> {
    let item = Item::new("item1", [1.0, 1.0, 1.0]);
    let items = (0..=27).map(|_| item.clone()).collect();
    let res = packing_algorithm(Bin::new([3.0, 3.0, 3.0]), &items)?;
    assert_eq!(res, vec![["item1"; 27].to_vec(), vec!["item1"]]);
    Ok(())
}

#[test]
fn test_odd_sizes() -> Result<()> {
    let item_1 = Item::new("item1", [3.0, 8.0, 10.0]);
    let item_2 = Item::new("item2", [1.0, 2.0, 5.0]);
    let item_3 = Item::new("item3", [1.0, 2.0, 2.0]);
    let items = vec![item_1, item_2.clone(), item_2, item_3];
    let res = packing_algorithm(Bin::new([10.0, 20.0, 20.0]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_odd_sizes_unordered() -> Result<()> {
    // test odd sized items will be sorted to fit.
    let item_1 = Item::new("item1", [3.0, 8.0, 10.0]);
    let item_2 = Item::new("item2", [1.0, 2.0, 5.0]);
    let item_3 = Item::new("item3", [1.0, 2.0, 2.0]);
    let items = vec![item_3, item_2.clone(), item_1, item_2];
    let res = packing_algorithm(Bin::new([10.0, 20.0, 20.0]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_slightly_larger_bin() -> Result<()> {
    let item = Item::new("item1", [4.0, 4.0, 12.0]);
    let items = vec![item.clone(), item];
    // let res = packing_algorithm(Bin::new([5.0, 8.0, 12.0]), &items)?;
    let res = packing_algorithm(Bin::new([4.0, 8.0, 12.0]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item1"]]);
    Ok(())
}

#[test]
fn test_pack_3_bins() -> Result<()> {
    let item = Item::new("item1", [4.0, 4.0, 12.0]);
    let items = vec![item.clone(), item.clone(), item];
    let res = packing_algorithm(Bin::new([4.0, 4.0, 12.0]), &items)?;
    assert_eq!(res, vec![vec!["item1"], vec!["item1"], vec!["item1"]]);
    Ok(())
}

#[test]
fn test_dim_over_2() -> Result<()> {
    // test that when length of item <= length of bin / 2 it packs along longer # edge

    let item = Item::new("item1", [3.0, 4.0, 5.0]);
    let items = (0..=3).map(|_| item.clone()).collect();
    let res = packing_algorithm(Bin::new([6.0, 8.0, 10.0]), &items)?;
    assert_eq!(res, vec![["item1"; 4].to_vec()]);
    Ok(())
}

#[test]
fn test_odd_sizes_again() -> Result<()> {
    // test items with different dimensions will be rotated to fit into one bin

    let item_1 = Item::new("item1", [1.0, 18.0, 19.0]);
    let item_2 = Item::new("item2", [17.0, 18.0, 18.0]);
    let item_3 = Item::new("item3", [1.0, 17.0, 18.0]);
    let items = vec![item_1, item_2, item_3];
    let res = packing_algorithm(Bin::new([18.0, 18.0, 19.0]), &items)?;
    assert_eq!(res, vec![vec!["item1", "item2", "item3"]]);
    Ok(())
}

#[test]
fn test_100_items_inexact_fit() -> Result<()> {
    // test many items into one bin with inexact fit

    let item = Item::new("item1", [5.0, 5.0, 5.0]);
    let items = (0..100).map(|_| item.clone()).collect();
    let res = packing_algorithm(Bin::new([51.0, 51.0, 6.0]), &items)?;
    assert_eq!(res.len(), 1);
    Ok(())
}

#[test]
fn test_100_items_inexact_fit_2_bins() -> Result<()> {
    // test many items separated into 2 bins with exact fit

    let item = Item::new("item1", [5.0, 5.0, 5.0]);
    let items = (0..100).map(|_| item.clone()).collect();
    let res = packing_algorithm(Bin::new([25.0, 10.0, 25.0]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res.first().map(|packed| packed.len()), Some(50));
    assert_eq!(res.last().map(|packed| packed.len()), Some(50));
    Ok(())
}

#[test]
fn test_big_die_and_serveral_decks_of_cards() -> Result<()> {
    let deck = Item::new("deck", [2.0, 8.0, 12.0]);
    let die = Item::new("die", [8.0, 8.0, 8.0]);
    let items = vec![deck.clone(), deck.clone(), die, deck.clone(), deck];
    let res = packing_algorithm(Bin::new([8.0, 8.0, 12.0]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res, vec![["deck"; 4].to_vec(), vec!["die"]]);
    Ok(())
}

#[test]
fn test_tight_fit_many_oblong() -> Result<()> {
    // tests a tight fit for non-cubic items

    let item_1 = Item::new("item1", [1.0, 2.0, 3.0]);
    let items = (0..107).map(|_| item_1.clone()).collect();
    let res = packing_algorithm(Bin::new([8.0, 9.0, 9.0]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res, vec![["item1"; 106].to_vec(), vec!["item1"]]);
    Ok(())
}

#[test]
fn test_tight_fit_many_oblong_inexact() -> Result<()> {
    // tests that the algorithm remains at least as accurate as it already is. If it were perfect,
    // the first bin would have 48 in it

    let item_1 = Item::new("item1", [1.0, 2.0, 3.0]);
    let items = (0..49).map(|_| item_1.clone()).collect();
    let res = packing_algorithm(Bin::new([4.0, 8.0, 9.0]), &items)?;
    assert_eq!(res.len(), 2);
    assert!(res.first().map(|packed| packed.len()) >= Some(44));
    Ok(())
}

#[test]
fn test_flat_bin() -> Result<()> {
    let item_1 = Item::new("item1", [1.25, 7.0, 10.0]);
    let items = vec![item_1.clone(), item_1.clone(), item_1];
    let res = packing_algorithm(Bin::new([3.5, 9.5, 12.5]), &items)?;
    assert_eq!(res.len(), 2);
    assert_eq!(res.first().map(|packed| packed.len()), Some(2));
    Ok(())
}

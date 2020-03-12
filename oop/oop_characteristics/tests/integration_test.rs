use oop_characteristics;

#[test]
fn it_can_read_averaged_collection() {
    let avg_col = oop_characteristics::AveragedCollection::from(
        vec![1, 2, 3]
    );

    assert_eq!(avg_col.average(), 2.0);
}
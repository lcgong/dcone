use mstruct::DomainUtil;

use mstruct::Error;

#[test]
fn domain_gen() -> Result<(), Error> {

    let domain = DomainUtil::new();
    domain
        .root()
        .set_empty_map()?
        .set_map_item("a")?
        .set_map_item("b")?
        .set_item("x", 10)?
        .set_item("y", 20)?
        ;

    println!("xxx {:?}", domain.root());


    let domain = DomainUtil::new();
    
    domain
        .root()
        .set_empty_list()?
        .push_item(10)
        .push_map_item()
        .push_list_item()
        .focus(1)?
        .set_item("a", 100)?
        .navigate("..#2")?
        .push_item(101)
        ;
    
    println!("xxx {:?}", domain.root());


    // println!("yyy {:?}", domain.navigate("#1/a"));

    assert_eq!(domain.navigate("#1/a")?.to_i64(), 100);

    domain.navigate("#1/a")?.set_value(200)?;
    
    println!("yyy {:?}", domain.navigate("#1/a"));

    Ok(())
}
use mstruct::{CellBuilder, Domain, DomainRootSetter, ValueBuilder};

#[test]
fn map() {
    let domain = Domain::new();

    // let root = domain.get_root();

    println!("root: {:?}", domain.get_root());

    domain.set_root(domain.value("THIS_ROOT"));
    println!("root: {:?}", domain.get_root());

    let list1 = domain.new_list();
    let list1 = list1
        .push(domain.value(10))
        .push(domain.value(20))
        .push(domain.value(30));
    let list1 = list1.set_value(1, domain.value("B2"));

    println!("{:?}", list1);

    let map1 = domain
        .new_map()
        .set_item("A".to_string(), domain.value(100))
        .set_item("B".to_string(), domain.value("B20"));

    println!("{:?}", map1);
}

#[test]
fn set_root() {
    let domain = Domain::new();

    assert!(domain.get_root().is_none());

    domain.set_root(domain.value("THIS_ROOT"));
    assert_eq!(domain.get_root().to_string(), "THIS_ROOT");

    let map = domain
        .set_root(domain.new_map())
        .set_item("A".to_string(), domain.value(10))
        .set_item("B".to_string(), domain.value(20));

    // println!("{:?}", map);
    domain.set_root(map);


    let list = domain
        .set_root(domain.new_list())
        .push(domain.value(10))
        .push(domain.value(20));

    // println!("{:?}", list);
    domain.set_root(list);

    for (i, r) in domain.logger.log.borrow().iter().enumerate() {
        println!("[{}]\t{:?}", i, r);
    }

    
}


mod error;
mod access_key;
mod focus;
mod turn_to;
mod locator;


pub use access_key::{AccessKey, CircularZeroIndex};
pub use focus::Focus;
pub use locator::{FocusLocator, AncestorIter};
pub use turn_to::FocusTurnTo;
pub use error::{AccessPathError, PathParsingError, OverFocusError};

//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::focus::{AccessKey, Focus, FocusLocator, FocusTurnTo};
    use crate::focus::AccessPathError;

    #[test]
    fn focus() {
        let f = Focus::new();

        let f1_1 = f.clone().focus(AccessKey::Index(1));
        let f1_1_a = f1_1.clone().focus(AccessKey::Key("A".to_string()));
        assert_eq!(f1_1.get_access_key(), AccessKey::Index(1));
        assert_eq!(f1_1_a.get_access_key(), AccessKey::Key("A".to_string()));

        let f2_1 = f.clone().focus(AccessKey::Index(1));
        let f2_1_a = f2_1.clone().focus(AccessKey::Key("A".to_string()));
        let f2_1_b = f2_1.clone().focus(AccessKey::Key("B".to_string()));
        assert!(std::sync::Arc::ptr_eq(&f1_1_a, &f2_1_a));
        assert_eq!(f1_1_a, f2_1_a);
        assert_ne!(f1_1_a, f2_1_b);
    }

    #[test]
    fn ancestors() {
        

        let f = Focus::new().focus("a").focus("b").focus("c");
        
        let ancestors = f.ancestors().collect::<Vec<&Arc<Focus>>>();

        assert_eq!(ancestors[0].get_access_key(), AccessKey::from("c"));
        assert_eq!(ancestors[1].get_access_key(), AccessKey::from("b"));
        assert_eq!(ancestors[2].get_access_key(), AccessKey::from("a"));
        assert_eq!(ancestors[3].get_access_key(), AccessKey::None);

        assert!(std::sync::Arc::ptr_eq(f.get_root(), ancestors[3]));
        assert!(std::sync::Arc::ptr_eq(&f.get_root().clone(), ancestors[3]));

        println!("Root: {:?}", f.get_root());
    }

    fn assert_turn_to(left_path: &str, right_path: &str) {

        let focus = Focus::new().turn_to(left_path).ok().unwrap();
        assert_eq!(focus.access_path(), right_path);
    }    

    fn assert_overfocus_error(left_path: &str) {

        assert!(match Focus::new().turn_to(left_path) {
            Ok(_) => false,
            Err(err) => match err {
                AccessPathError::OverFocus(_) => true,
                _=> false
            }
        });
    }

    #[test]
    fn access_path_1() {

        assert_eq!(Focus::new().access_path(), "/");

        assert_turn_to("a", "/a");
        assert_turn_to("a/b", "/a/b");
        assert_turn_to("/a/b", "/a/b");
        assert_turn_to("#1#2#3#4", "/#1#2#3#4");
        assert_turn_to("#1#2/#3#4", "/#1#2#3#4");

        assert_turn_to("/a/b/c/../../d", "/a/d");
       
        assert_overfocus_error("..");
        assert_overfocus_error("../../d");
        assert_overfocus_error("/a/../../d");

    }

    #[test]
    fn access_path_2() {

        let f = Focus::new();

        let f2 = f
            .clone()
            .turn_to("/a1/b2#123/c3")
            .ok()
            .unwrap();

        let f3 = f
            .clone()
            .turn_to("a1/b2")
            .turn_to("#123")
            .turn_to("c3".to_string())
            .ok()
            .unwrap();

        assert!(std::sync::Arc::ptr_eq(&f2, &f3));
        assert_eq!(f2, f3);
        assert_eq!(f3.access_path(), "/a1/b2#123/c3");

        assert_eq!(
            Focus::new()
                .focus("a")
                .focus("b")
                .focus("c")
                .access_path(),
            "/a/b/c"
        );

        assert_eq!(
            Focus::new()
                .focus(1)
                .focus(2)
                .focus(-3)
                .focus(4)
                .access_path(),
            "/#1#2#-3#4"
        );
    }
    #[test]
    fn foreach_directions() {

        let mut focuses: Vec<Arc<Focus>> = Vec::new();

        let r = Focus::new();
        
        focuses.push(r.turn_to("/a1/a").ok().unwrap());
        focuses.push(r.turn_to("/a2/a").ok().unwrap());
        focuses.push(r.turn_to("/a3/a").ok().unwrap());
        focuses.push(r.turn_to("/a4/a").ok().unwrap());


        // println!("{:?}", r.get_directions());
        assert_eq!(r.get_direction_keys().len(), 4);
        
        let mut directions : Vec<Arc<Focus>> = Vec::new();
        r.foreach_directions(|focus| {
            directions.push(focus.clone());
        });
        assert_eq!(directions.len(), 4);
        // println!("dd: {:?}", directions);

        let mut directions : Vec<Arc<Focus>> = Vec::new();
        r.foreach_directions(|focus| {
            directions.push(focus.clone());
            focus.foreach_directions(|focus| {
                directions.push(focus.clone());
            });
        });
        assert_eq!(directions.len(), 8);
        // println!("dd: {:?}", directions);
    }

    #[test]
    fn auto_drop() {

        let r = Focus::new();
        {
            let a = r.focus("a");
            let b = r.focus("b");
            let c = r.focus("c");
            {
                let a1 = a.focus("a1");
                assert_eq!(a.get_direction_keys().len(), 1);
                {
                    a1.focus("a2");
                    println!("-[1]--------------");
                }
                
                let _b2 = b.focus("b1");
                let _c3 = c.focus("c1");

                assert_eq!(a1.get_direction_keys().len(), 0);
                println!("-[2]--------------");
            }
            assert_eq!(r.get_direction_keys().len(), 3);
            assert_eq!(a.get_direction_keys().len(), 0);
            println!("-[3]--------------");
        }    
        assert_eq!(r.get_direction_keys().len(), 0);
    
    }

}

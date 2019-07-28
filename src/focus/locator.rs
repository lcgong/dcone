
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use super::access_key::AccessKey;
use super::focus::Focus;
use super::turn_to::FocusTurnTo;

pub trait FocusLocator: FocusTurnTo {
    fn focus<T: Into<AccessKey>>(&self, acccess_key: T) -> Arc<Focus>;

    fn get_root<'a>(&'a self) -> &'a Arc<Focus>;

    fn get_parent<'a>(&'a self) -> Option<&'a Arc<Focus>>;

    fn ancestors(&self) -> AncestorIter;

    fn get_access_key(&self) -> AccessKey;

    fn access_path(&self) -> String;

    fn get_direction_keys<'a>(&'a self) -> Vec<AccessKey>;

    fn foreach_directions<F>(&self, f: F) where F: FnMut(&Arc<Focus>);
}


impl FocusLocator for Arc<Focus> {
    fn focus<T: Into<AccessKey>>(&self, acccess_key: T) -> Arc<Focus> {
        
        // let mut directions = self.directions;

        let access_key = acccess_key.into();

        let mut directions = self.directions.write().unwrap();

        if let Some(existing_focus) = directions.get(&access_key) {
            if let Some(existing_focus) = existing_focus.upgrade() {
                return existing_focus;
            }
        }
        let new_focus = Arc::new(Focus {
            parent_focus: Some(self.clone()),
            access_key: access_key.clone(),
            directions: RwLock::new(HashMap::new()),
            // directions: HashMap::new(),
        });

        directions.insert(access_key, Arc::downgrade(&new_focus));

        new_focus
    }

    fn get_access_key(&self) -> AccessKey {
        self.access_key.clone()
    }

    fn get_root<'a>(&'a self) -> &'a Arc<Focus> {
        
        let mut current = self;
        
        while let Some(ref parent_focus) = current.parent_focus {
            current = parent_focus
        }
        
        current
    }

    fn get_parent<'a>(&'a self) -> Option<&'a Arc<Focus>> {
        match self.parent_focus {
            Some(ref parent) => Some(parent),
            None => None
        }
    }

    fn ancestors<'a>(&'a self) -> AncestorIter<'a> {
        AncestorIter {
            next: Some(self),
        }
    }

    fn access_path(&self) -> String {
        let mut segments = self.ancestors();

        let mut path = String::new();

        if let Some(focus) = segments.next() {
            let mut is_key_previous;
            match focus.access_key {
                AccessKey::Key(ref key) => {
                    path.push_str(key);
                    is_key_previous = true;
                }
                AccessKey::Index(index) => {
                    path.push('#');
                    path.push_str(index.to_string().as_str());
                    is_key_previous = false;
                }
                AccessKey::None => {
                    return "/".to_string();
                }
            };

            loop {
                if let Some(focus) = segments.next() {
                    match focus.get_access_key() {
                        AccessKey::Key(ref key) => {
                            if is_key_previous {
                                path.insert(0, '/');
                            }
                            path.insert_str(0, key);
                            is_key_previous = true;
                        }
                        AccessKey::Index(index) => {
                            if is_key_previous {
                                path.insert(0, '/');
                            }
                            
                            path.insert_str(0, index.to_string().as_str());
                            path.insert(0, '#');

                            is_key_previous = false;
                        }
                        AccessKey::None => {
                            path.insert(0, '/');
                            is_key_previous = false;
                        }
                    };
                } else {
                    break;
                }
            }
        }
        path
    }

    fn get_direction_keys<'a>(&'a self) -> Vec<AccessKey> {

        let directions = self.directions.read().unwrap();

        directions.iter()
            .map(|(access_key, _)| {access_key.clone()})
            .collect::<Vec<AccessKey>>()
    }

    fn foreach_directions<F>(&self, mut func: F)
        where F: FnMut(&Arc<Focus>) {

        let directions = self.directions.read().unwrap();
        for (_, ref weak_focus) in directions.iter() {
            if let Some(ref focus) = weak_focus.upgrade() {
                func(focus)
            }
        }
    }

}


pub struct AncestorIter<'a> {
    next: Option<&'a Arc<Focus>>,
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = &'a Arc<Focus>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(focus_ref) = self.next {
            let current_ref = focus_ref;

            self.next = match current_ref.parent_focus {
                Some(ref parent_focus) => Some(parent_focus),
                None => None
            };

            Some(&current_ref)
        } else {
            None
        }
    }
}
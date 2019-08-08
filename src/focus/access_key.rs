

pub type CircularZeroIndex = isize;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AccessKey {
    None,
    Key(String),
    /// circular zere-based index, 0 means the first, -1 the last.
    Index(CircularZeroIndex),
}


impl From<String> for AccessKey {
    
    #[inline]
    fn from(key: String) -> AccessKey {
        AccessKey::Key(key)
    }
}

impl From<&str> for AccessKey {
    
    #[inline]
    fn from(key: &str) -> AccessKey {
        AccessKey::Key(key.to_string())
    }
}

impl From<isize> for AccessKey {
    
    #[inline]
    fn from(index: isize) -> AccessKey {
        AccessKey::Index(index)
    }
}

impl From<i64> for AccessKey {
    
    #[inline]
    fn from(index: i64) -> AccessKey {
        AccessKey::Index(index as isize)
    }
}

impl From<i32> for AccessKey {
    
    #[inline]
    fn from(index: i32) -> AccessKey {
        AccessKey::Index(index as isize)
    }
}



// impl std::cmp::Ord for AccessKey {
//     fn cmp(&self, other: &AccessKey) -> std::cmp::Ordering {
//         match (self, other) {
//             (AccessKey::Key(a), AccessKey::Key(b)) => a.cmp(b),
//             (AccessKey::Index(a), AccessKey::Index(b)) => a.cmp(b),
//         }
//     }
// }



impl ::std::fmt::Debug for AccessKey {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        // println!("1111 {:?}", self);
        match &self {
            AccessKey::Key(key) => {
                fmt.write_str("Key(")?;
                key.fmt(fmt)?;
                fmt.write_str(")")
            },
            AccessKey::Index(index) => {
                fmt.write_str("Index(")?;
                index.fmt(fmt)?;
                fmt.write_str(")")
            },
            AccessKey::None => fmt.write_str("None"),
        }
    }
}

impl std::fmt::Display for AccessKey {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match &self {
            AccessKey::Key(key) => {
                fmt.write_str("Key(")?;
                key.fmt(fmt)?;
                fmt.write_str(")")
            },
            AccessKey::Index(index) => {
                fmt.write_str("Index(")?;
                index.fmt(fmt)?;
                fmt.write_str(")")
            },
            AccessKey::None => fmt.write_str("None"),
        }
    }
}

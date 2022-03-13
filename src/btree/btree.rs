pub struct Node {
    pub data: Vec<i64>,
    pub right: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
}

pub struct MJBtree {
    pub btree: Node,
}

/// 1st prototype btree for i64
/// max degree 3
impl MJBtree {
    pub fn new() -> MJBtree {
        return MJBtree {
            btree: Node {
                data: vec![0],
                right: None,
                left: None,
            },
        };
    }

    // pub fn insert() {}
    //
    // pub fn find(number: i64) {}
    //
    // pub fn delete(number: i64) {}
    //
    // pub fn print() {}
}

#[test]
fn btree_test() {
    let mut btr = MJBtree::new();

    /// add node
    btr.btree.right = Option::from(Box::from(Node {
        data: vec![5],
        right: None,
        left: None
    }));


    println!("a contain {:?}", btr.btree.data);

    /// right node
    match btr.btree.right {
        None => {
            println!("Null right node");
        }
        Some(r) => {
            println!("a contain {:?}", r.btree.data);
        }
    }
}

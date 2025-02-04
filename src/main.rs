/// return the position of the key where nkey is or should be added.
fn binary_search<T: Copy + Ord>(v: &[T], nkey: T) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut l: usize = 0;
    let mut r: usize = v.len() - 1;
    let mut mid = 0;
    while l < r {
        mid = (l + r) / 2;
        match v[mid] {
            m if m > nkey => {
                if r != 0 {
                    r = mid - 1;
                }
            }
            m if m <= nkey => {
                l = mid + 1;
            }
            _ => unreachable!(),
        }
    }
    mid
}

struct BTree<T> {
    root: Option<BNode<T>>,
    order: usize,
}

impl<T: Ord + Copy + Clone> BTree<T> {
    fn new(order: usize) -> BTree<T> {
        Self { order, root: None }
    }

    fn insert(&mut self, nkey: T) {
        if self.root.is_none() {
            self.root = Some(BNode::new_fill(
                vec![nkey],
                (0..self.order / 2 + 1).map(|_| BNode::new()).collect(),
            ));
        } else {
            let root = self.root.as_mut().unwrap();
            if let Some((median, new_child_node)) = BTree::_insert(root, nkey, self.order) {
                let old_root = self.root.take().unwrap();
                let new_root = BNode::new_fill(vec![median], vec![old_root, *new_child_node]);
                self.root = Some(new_root);
            }
        }
    }

    fn _insert(curr: &mut BNode<T>, nkey: T, order: usize) -> Option<(T, Box<BNode<T>>)> {
        if BNode::is_leaf(curr) {
            let pos = binary_search(&curr.keys, nkey);
            curr.keys.insert(pos, nkey);
            if curr.keys.len() < 2 * order - 1 {
                return None;
            }

            let mid = curr.keys.len() / 2;
            let median = curr.keys.remove(mid);
            let new_keys = if mid + 1 < curr.keys.len() {
                curr.keys.split_off(mid + 1)
            } else {
                vec![]
            };

            let new_children = if mid + 1 < curr.children.len() {
                curr.children.split_off(mid + 1)
            } else {
                vec![]
            };

            let new_node = BNode::new_fill(new_keys, new_children);
            let new_child = Box::new(new_node);
            return Some((median, new_child));
        }
        let mut child_index: usize = 0;
        for (i, &key) in curr.keys.iter().enumerate() {
            if key >= nkey {
                child_index = i;
                break;
            }
        }
        if let Some((median, new_child_node)) =
            BTree::_insert(&mut curr.children[child_index], nkey, order)
        {
            curr.keys.insert(child_index, median);
            curr.children.insert(child_index + 1, *new_child_node);

            if curr.keys.len() >= 2 * order - 1 {
                let mid = curr.keys.len() / 2;
                let median = curr.keys.remove(mid);
                let new_keys = curr.keys.split_off(mid);
                let new_children = curr.children.split_off(mid + 1);

                let new_node = BNode::new_fill(new_keys, new_children);
                return Some((median, Box::new(new_node)));
            }
        }

        None
    }
}

struct BNode<T> {
    keys: Vec<T>,
    children: Vec<BNode<T>>,
}

impl<T: Copy + Ord + Clone> BNode<T> {
    fn is_leaf(node: &BNode<T>) -> bool {
        node.children.is_empty()
    }
    fn new() -> Self {
        Self {
            keys: vec![],
            children: vec![],
        }
    }

    fn new_fill(keys: Vec<T>, children: Vec<BNode<T>>) -> Self {
        Self { keys, children }
    }
}

fn main() {
    let mut tree: BTree<i32> = BTree::new(2);
    tree.insert(10);
    tree.insert(11);
    tree.insert(12);
    tree.insert(13);
}

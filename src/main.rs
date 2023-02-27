use std::borrow::BorrowMut;


enum TraversalCrumb {
    TookLeft(usize),
    TookRight(usize),
}

fn print_pre_order(t: &Tree) {

    let mut cur = t.traverse();
    let mut out = Vec::new();
    let mut crumbs = Vec::new();

    loop {
        out.push(cur.data());
        
        if cur.left().is_some() {
            crumbs.push(TraversalCrumb::TookLeft(cur.cursor));
            cur.go_left();
        } else if cur.right().is_some() {
            crumbs.push(TraversalCrumb::TookRight(cur.cursor));
            cur.go_right();
        }
        
        let mut next = false;
        while let Some(crumb) = crumbs.pop() {
            match crumb {
                TraversalCrumb::TookLeft(index) => {
                    cur.goto(index);
                    if cur.right().is_some() {
                        crumbs.push(TraversalCrumb::TookRight(cur.cursor));
                        cur.go_right();
                        next = true;
                        break;
                    }
                },
                TraversalCrumb::TookRight(index) => {
                    cur.goto(index);
                },
            }
        }
        if !next {
            break;
        }
    }

    println!("{}", out.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn print_post_order(t: &Tree) {
    let mut cur = t.traverse();
    let mut out = Vec::new();
    let mut crumbs = Vec::new();

    loop {      
        if cur.left().is_some() {
            crumbs.push(TraversalCrumb::TookLeft(cur.cursor));
            cur.go_left();
            continue;
        } else if cur.right().is_some() {
            crumbs.push(TraversalCrumb::TookRight(cur.cursor));
            cur.go_right();
            continue;
        }

        out.push(cur.data());

        let mut next = false;
        while let Some(crumb) = crumbs.pop() {
            match crumb {
                TraversalCrumb::TookLeft(index) => {
                    cur.goto(index);
                    if cur.right().is_some() {
                        crumbs.push(TraversalCrumb::TookRight(cur.cursor));
                        cur.go_right();
                        next = true;
                        break;
                    } else {
                        out.push(cur.data());
                    }
                },
                TraversalCrumb::TookRight(index) => {
                    cur.goto(index);
                    out.push(cur.data());
                },
            }
        }
        if !next {
            break;
        }
    }

    println!("{}", out.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn main() -> std::io::Result<()> {
    let mut len = String::new();
    let mut input_str = String::new();
    
    std::io::stdin().read_line(&mut len)?;
    std::io::stdin().read_line(&mut input_str)?;
    
    let input: Vec<i32> = input_str
        .split(" ")
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    
    let mut tree = Tree::new();

    for data in input {
        tree.insert(data);
    }

    print_post_order(&tree);
    
    Ok(())
}

struct Node {
    data: i32,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
    root: usize,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: 0
        }
    }

    fn traverse(&self) -> NodeTraverse {
        NodeTraverse {
            tree: self,
            cursor: self.root,
        }
    }

    fn insert(&mut self, data: i32) {
        let new_node = Node::new(data);
        self.nodes.push(new_node);
        let new_index = self.nodes.len() - 1;
        
        if self.nodes.len() == 1 {
            self.root = new_index;
            return;
        }
        
        let mut cursor = self.root;
        loop {
            let c = self.nodes.get_mut(cursor).unwrap();

            let new_cursor_opt = if data < c.data {
                c.left.borrow_mut()
            } else {
                c.right.borrow_mut()
            };

            if let Some(new_cursor) = new_cursor_opt {
                cursor = *new_cursor;
            } else {
                *new_cursor_opt = Some(new_index);
                break;
            }
        }
    }
}

struct NodeTraverse<'a> {
    tree: &'a Tree,
    cursor: usize,
}

impl<'a> NodeTraverse<'a> {
    fn goto(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    fn data(&self) -> i32 {
        self.cur().unwrap().data
    }

    fn left(&self) -> Option<usize> {
        self.cur().unwrap().left
    }

    fn right(&self) -> Option<usize> {
        self.cur().unwrap().right
    }

    fn go_left(&mut self) {
        self.cursor = self.left().unwrap();
    }

    fn go_right(&mut self) {
        self.cursor = self.right().unwrap();
    }

    fn cur(&self) -> Option<&Node> {
        self.tree.nodes.get(self.cursor)
    }
}

#[cfg(test)]
mod test {
    use crate::Tree;

    #[test]
    fn test_tree_works() {
        let mut t = Tree::new();

        t.insert(3);
        t.insert(2);
        t.insert(1);
        t.insert(4);

        let l1 = t.nodes.get(t.root).unwrap().left.unwrap();
        let l2 = t.nodes.get(l1).unwrap().left.unwrap();
        let node_1 = t.nodes.get(l2).unwrap();

        assert_eq!(node_1.data, 1);
    }

    #[test]
    fn test_traverse() {
        let mut t = Tree::new();

        t.insert(2);
        t.insert(4);
        t.insert(6);
        t.insert(3);
        t.insert(1);

        let mut traverse = t.traverse();

        traverse.go_right();
        traverse.go_left();
        
        assert_eq!(traverse.data(), 3);
    }

    #[test]
    fn test_complex_tree() {
        let mut t = Tree::new();

        let input = [1, 14, 3, 7, 4, 5, 15, 6, 13, 10, 11, 2, 12, 8, 9];

        for d in input {
            t.insert(d);
        }

        let mut trav = t.traverse();
        trav.go_right();
        assert_eq!(trav.data(), 14);
        trav.go_left();
        assert_eq!(trav.data(), 3);
        trav.go_right();
        assert_eq!(trav.data(), 7);
        trav.go_right();
        assert_eq!(trav.data(), 13);
    }
}

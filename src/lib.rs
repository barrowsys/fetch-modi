#[macro_use]
extern crate str_macro;
use std::cmp::Ordering;
use std::fmt;

pub enum InsertResult {
    Success,
    SuccessBut(Vec<String>),
    CollisionDetected,
}

pub enum FetchResult {
    Success(String),
    SuccessBut(String, Vec<String>),
    Empty,
}

pub mod john {
    use super::*;
    pub struct Modus {
        pub items: Vec<String>,
        pub size: usize,
    }

    impl Modus {
        pub fn new(size: usize) -> Modus {
            if size < 1 {
                panic!("size < 1!!!!")
            }
            Modus {
                items: vec![],
                size: size,
            }
        }
        pub fn queue_put(&mut self, item: &str) -> InsertResult {
            self.items.insert(0, str!(item));
            if self.items.len() > self.size {
                InsertResult::SuccessBut(vec![self.items.pop().unwrap()])
            } else {
                InsertResult::Success
            }
        }
        pub fn queue_take(&mut self) -> FetchResult {
            match self.items.pop() {
                Some(item) => FetchResult::Success(item),
                None => FetchResult::Empty,
            }
        }
        pub fn stack_put(&mut self, item: &str) -> InsertResult {
            self.items.insert(0, str!(item));
            if self.items.len() > self.size {
                InsertResult::SuccessBut(vec![self.items.pop().unwrap()])
            } else {
                InsertResult::Success
            }
        }
        pub fn stack_take(&mut self) -> FetchResult {
            if self.items.len() == 0 {
                FetchResult::Empty
            } else {
                FetchResult::Success(self.items.remove(0))
            }
        }
    }

    impl fmt::Display for Modus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Captchalogue Deck:")?;
            for index in 0..self.size {
                let item = self.items.get(index);
                if let Some(item) = item {
                    write!(f, " |{}|", item)?;
                } else {
                    write!(f, " |_|")?;
                }
            }
            write!(f, "")
        }
    }
}

pub mod rose {
    use super::*;
    pub struct TreeBranch {
        pub trunk: String,
        pub lbranch: Option<Box<TreeBranch>>,
        pub rbranch: Option<Box<TreeBranch>>,
    }
    pub struct Modus {
        pub root: TreeBranch,
    }
    enum TakeResult {
        Return(String),
        Found(bool),
        Empty,
    }

    impl TreeBranch {
        fn new(trunk: &str) -> TreeBranch {
            TreeBranch {
                trunk: str!(trunk),
                lbranch: None,
                rbranch: None,
            }
        }
        fn flatten(&self) -> Vec<String> {
            fn flatten_branch(
                buffer: &mut Vec<String>,
                delim: &str,
                branch: &Option<Box<TreeBranch>>,
            ) {
                match branch {
                    Some(b) => {
                        let branch_vec: Vec<String> = b.flatten();
                        for branch_item in branch_vec.iter() {
                            buffer.push(format!(" {} {}", delim, branch_item));
                        }
                    }
                    None => (),
                }
            }
            let mut strings = Vec::<String>::new();
            strings.push(format!("-{}", self.trunk.clone()));
            flatten_branch(&mut strings, "L", &self.lbranch);
            flatten_branch(&mut strings, "R", &self.rbranch);
            strings
        }
        fn add(&mut self, item: &str) {
            match str!(item).cmp(&self.trunk) {
                Ordering::Less => {
                    if let Some(lbranch) = &mut self.lbranch {
                        lbranch.add(item)
                    } else {
                        self.lbranch = Some(Box::new(TreeBranch::new(item)));
                    }
                }
                Ordering::Greater => {
                    if let Some(rbranch) = &mut self.rbranch {
                        rbranch.add(item)
                    } else {
                        self.rbranch = Some(Box::new(TreeBranch::new(item)));
                    }
                }
                Ordering::Equal => {}
            }
        }
        fn leaves(&self) -> Vec<&str> {
            if self.lbranch.is_none() && self.rbranch.is_none() {
                return vec![&self.trunk[..]];
            } else {
                let mut vec: Vec<&str> = vec![];
                if let Some(lb) = &self.lbranch {
                    vec.append(&mut lb.leaves());
                }
                if let Some(rb) = &self.rbranch {
                    vec.append(&mut rb.leaves());
                }
                vec
            }
        }
        fn take(&mut self, item: &str, is_root: bool) -> TakeResult {
            match str!(item).cmp(&self.trunk) {
                Ordering::Less => {
                    if let Some(branch) = &mut self.lbranch {
                        match branch.take(item, false) {
                            TakeResult::Found(_) => {
                                let rstr = branch.trunk.clone();
                                self.lbranch = None;
                                TakeResult::Return(rstr)
                            }
                            TakeResult::Return(rstr) => TakeResult::Return(rstr),
                            TakeResult::Empty => TakeResult::Empty,
                        }
                    } else {
                        TakeResult::Empty
                    }
                }
                Ordering::Greater => {
                    if let Some(branch) = &mut self.rbranch {
                        match branch.take(item, false) {
                            TakeResult::Found(_) => {
                                let rstr = branch.trunk.clone();
                                self.rbranch = None;
                                TakeResult::Return(rstr)
                            }
                            TakeResult::Return(rstr) => TakeResult::Return(rstr),
                            TakeResult::Empty => TakeResult::Empty,
                        }
                    } else {
                        TakeResult::Empty
                    }
                }
                Ordering::Equal => TakeResult::Found(is_root),
            }
        }
    }

    impl Modus {
        pub fn new(root: &str) -> Modus {
            Modus {
                root: TreeBranch {
                    trunk: str!(root),
                    lbranch: None,
                    rbranch: None,
                },
            }
        }
        pub fn add(&mut self, item: &str) {
            self.root.add(item);
        }
        pub fn test() -> Modus {
            let mut modus = Modus {
                root: TreeBranch::new("Ipsum"),
            };
            modus.add("Lorem");
            modus.add("Dolor");
            modus.add("Set");
            modus.add("Amat");
            modus.add("Exaltus");
            modus.add("Joshuus");
            modus
        }
        pub fn takeables(&self) -> Vec<&str> {
            self.root.leaves()
        }
        pub fn take(&mut self, item: &str) -> FetchResult {
            match self.root.take(item, true) {
                TakeResult::Empty => FetchResult::Empty,
                TakeResult::Return(rstr) => FetchResult::Success(rstr),
                _ => FetchResult::Empty,
            }
        }
    }

    impl fmt::Display for Modus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Tree Modus")?;
            let vec = self.root.flatten();
            for v in vec.iter() {
                writeln!(f, "{}", v)?;
            }
            write!(f, "")
        }
    }
}

pub mod dave {
    use super::*;
    pub struct Modus<T>
    where
        T: Fn(&str) -> usize,
    {
        pub items: [Option<String>; 10],
        pub hash_function: T,
    }

    pub fn hash_default(key: &str) -> usize {
        let mut total = 0;
        for c in key.chars() {
            total += match c {
                'a' => 1,
                'e' => 1,
                'i' => 1,
                'o' => 1,
                'u' => 1,
                'y' => 1,
                _ => 2,
            }
        }
        total % 10
    }

    impl<T> Modus<T>
    where
        T: Fn(&str) -> usize,
    {
        pub fn new(func: T) -> Modus<T> {
            Modus {
                items: [None, None, None, None, None, None, None, None, None, None],
                hash_function: func,
            }
        }
        pub fn add(&mut self, keystr: &str) -> InsertResult {
            let key = (self.hash_function)(keystr);
            if self.items[key].is_none() {
                self.items[key] = Some(str!(keystr));
                InsertResult::Success
            } else {
                let old = vec![self.items[key].as_ref().unwrap().clone()];
                self.items[key] = Some(str!(keystr));
                InsertResult::SuccessBut(old)
            }
        }
        pub fn get(&mut self, key: &str) -> FetchResult {
            let key = (self.hash_function)(key);
            if self.items[key].is_none() {
                FetchResult::Empty
            } else {
                let val = self.items[key].as_ref().unwrap().clone();
                self.items[key] = None;
                FetchResult::Success(val)
            }
        }
    }

    impl<T> fmt::Display for Modus<T>
    where
        T: Fn(&str) -> usize,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Captchalogue Deck:")?;
            for index in 0..10 {
                let item = self.items[index].as_ref();
                if let Some(item) = item {
                    writeln!(f, "{} |{}|", index, item)?;
                } else {
                    writeln!(f, "{} |_|", index)?;
                }
            }
            write!(f, "")
        }
    }
}

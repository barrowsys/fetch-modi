#[macro_use]
extern crate str_macro;
use console::style;
use std::cmp::Ordering;
use std::fmt;

fn green(t: &str) -> console::StyledObject<&str> {
    style(t).green()
}

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

impl fmt::Display for InsertResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InsertResult::Success => write!(f, "Successfully Inserted"),
            InsertResult::SuccessBut(items) => {
                for i in items.iter() {
                    writeln!(f, "The {} was launched out of your sylladex!", green(i))?;
                }
                write!(f, "Successfully Inserted")
            }
            InsertResult::CollisionDetected => write!(f, "Collision Detected!"),
        }
    }
}
impl fmt::Display for FetchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchResult::Empty => write!(f, "Nothing to take!"),
            FetchResult::Success(item) => write!(f, "Removed {} from sylladex", green(item)),
            FetchResult::SuccessBut(item, items) => {
                for i in items.iter() {
                    writeln!(f, "The {} was launched out of your sylladex!", green(i))?;
                }
                write!(f, "Removed {} from sylladex", green(item))
            }
        }
    }
    //     match result {
    //         FetchResult::Empty => println!("nothing to take"),
    //         FetchResult::Success(item) => println!("Removed {} from sylladex", item),
    //         FetchResult::SuccessBut(_, _) => panic!("this shouldn't happen"),
    //     };
}

pub mod john {
    use super::*;
    pub enum Modi {
        Queue,
        Stack,
        Array,
        QueueStack,
        ArrayQueueStack,
    }
    pub struct QueueStackModus {
        pub items: Vec<String>,
        pub size: usize,
    }
    pub struct ArrayQSModus {
        pub array: Vec<QueueStackModus>,
        pub size: usize,
    }

    impl ArrayQSModus {
        pub fn new(size: usize) -> ArrayQSModus {
            if size < 1 {
                panic!("size < 1!!!!")
            }
            let mut vec = Vec::<QueueStackModus>::new();
            for _ in 0..size {
                vec.push(QueueStackModus::new(size));
            }
            ArrayQSModus { array: vec, size }
        }
        pub fn put(&mut self, index: usize, is_queue: bool, item: &str) -> InsertResult {
            if index >= self.size {
                InsertResult::CollisionDetected
            } else {
                let qs = self.array.get_mut(index).unwrap();
                if is_queue {
                    qs.queue_put(item)
                } else {
                    qs.stack_put(item)
                }
            }
        }
        pub fn get(&mut self, index: usize, is_queue: bool) -> FetchResult {
            if index >= self.size {
                FetchResult::Empty
            } else {
                let qs = self.array.get_mut(index).unwrap();
                if is_queue {
                    qs.queue_take()
                } else {
                    qs.stack_take()
                }
            }
        }
    }

    impl QueueStackModus {
        pub fn new(size: usize) -> QueueStackModus {
            if size < 1 {
                panic!("size < 1!!!!")
            }
            QueueStackModus {
                items: vec![],
                size,
            }
        }
        pub fn new_array(size: usize) -> QueueStackModus {
            if size < 1 {
                panic!("size < 1!!!!")
            }
            let mut vec = Vec::<String>::new();
            for _ in 0..size {
                vec.push(str!(""));
            }
            QueueStackModus { items: vec, size }
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
            if self.items.is_empty() {
                FetchResult::Empty
            } else {
                FetchResult::Success(self.items.remove(0))
            }
        }
        pub fn array_put(&mut self, index: usize, item: &str) -> InsertResult {
            if index > self.size {
                return InsertResult::CollisionDetected;
            }
            let old = self.items.remove(index);
            self.items.insert(index, str!(item));
            if old.is_empty() {
                InsertResult::Success
            } else {
                InsertResult::SuccessBut(vec![old])
            }
        }
        pub fn array_take(&mut self, index: usize) -> FetchResult {
            if index > self.size {
                return FetchResult::Empty;
            }
            let old = self.items.remove(index);
            self.items.insert(index, str!(""));
            if old.is_empty() {
                FetchResult::Success(old)
            } else {
                FetchResult::Empty
            }
        }
    }

    impl fmt::Display for QueueStackModus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for index in 0..self.size {
                let item = self.items.get(index);
                if let Some(item) = item {
                    if item.is_empty() {
                        write!(f, " |_|")?;
                    } else {
                        write!(f, " |{}|", green(item))?;
                    }
                } else {
                    write!(f, " |_|")?;
                }
            }
            write!(f, "")
        }
    }

    impl fmt::Display for ArrayQSModus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for index in 0..self.size {
                let arr = self.array.get(index).unwrap();
                writeln!(f, "{}: {}", index, arr)?;
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
        fn size(&self) -> usize {
            let mut rv = 1;
            if self.rbranch.is_some() {
                rv += self.rbranch.as_ref().unwrap().size();
            }
            if self.lbranch.is_some() {
                rv += self.lbranch.as_ref().unwrap().size();
            }
            rv
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
                        for branch_item in &branch_vec {
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
        fn compactify(&self) -> Vec<String> {
            fn compactify_branch(buffer: &mut Vec<String>, branch: &Option<Box<TreeBranch>>) {
                match branch {
                    Some(b) => {
                        let mut branch_vec: Vec<String> = b.compactify();
                        while !branch_vec.is_empty() {
                            buffer.push(branch_vec.remove(0));
                        }
                    }
                    None => (),
                }
            }
            let mut strings = Vec::<String>::new();
            strings.push(self.trunk.clone());
            compactify_branch(&mut strings, &self.lbranch);
            compactify_branch(&mut strings, &self.rbranch);
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
        fn depths(&self) -> Vec<u32> {
            fn depths_impl(branch: &TreeBranch, level: u32) -> Vec<u32> {
                if branch.lbranch.is_none() && branch.rbranch.is_none() {
                    return vec![level];
                } else {
                    let mut rtvl = Vec::<u32>::new();
                    if branch.lbranch.is_some() {
                        rtvl.append(&mut depths_impl(
                            &branch.lbranch.as_ref().unwrap(),
                            level + 1,
                        ));
                    }
                    if branch.rbranch.is_some() {
                        rtvl.append(&mut depths_impl(
                            &branch.rbranch.as_ref().unwrap(),
                            level + 1,
                        ));
                    }
                    rtvl
                }
            }
            depths_impl(self, 0)
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
                root: TreeBranch::new(root),
            }
        }
        pub fn add(&mut self, item: &str) -> InsertResult {
            self.root.add(item);
            InsertResult::Success
        }
        pub fn size(&self) -> usize {
            self.root.size()
        }
        pub fn takeables(&self) -> Vec<&str> {
            self.root.leaves()
        }
        pub fn take(&mut self, item: &str) -> FetchResult {
            match self.root.take(item, true) {
                TakeResult::Return(rstr) => FetchResult::Success(rstr),
                _ => FetchResult::Empty,
            }
        }
        pub fn take_root(&mut self) -> FetchResult {
            let rv = FetchResult::SuccessBut(self.root.trunk.clone(), self.root.compactify());
            self.root.lbranch = None;
            self.root.rbranch = None;
            self.root.trunk = String::from("");
            rv
        }
        pub fn balance(&mut self) {
            pub fn balance_impl(mut strings: Vec<String>) -> Vec<String> {
                let mut len = strings.len();
                if len < 2 {
                    return strings;
                }
                if strings.len() % 2 == 1 {
                    len -= 1;
                }
                let midpoint = len / 2;
                let mut new_nodes = Vec::<String>::new();
                new_nodes.push(strings.remove(midpoint));
                let half2 = strings.split_off(midpoint);
                new_nodes.append(&mut balance_impl(strings));
                new_nodes.append(&mut balance_impl(half2));
                new_nodes
            }
            let mut nodes = self.root.compactify();
            nodes.sort();
            let mut new_nodes = balance_impl(nodes);
            let mut new_tree = TreeBranch::new(&new_nodes.remove(0));
            for node in &new_nodes {
                new_tree.add(node);
            }
            self.root = new_tree;
        }
        pub fn autobalance(&mut self) -> bool {
            let mut depths = self.root.depths();
            if depths.len() == 1 {
                depths.insert(0, 0);
            }
            depths.sort();
            if depths.last().unwrap() - depths.first().unwrap() > 1 {
                self.balance();
                true
            } else {
                false
            }
        }
    }

    impl fmt::Display for Modus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Tree Modus")?;
            let vec = self.root.flatten();
            for v in &vec {
                write!(f, "\n{}", v)?;
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
        pub detect_collisions: bool,
    }

    pub fn c2v1(key: &str) -> usize {
        let mut total = 0;
        for c in key.to_lowercase().chars() {
            total += match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' => 1,
                ' ' => 0,
                _ => 2,
            }
        }
        total % 10
    }

    pub fn c1v2(key: &str) -> usize {
        let mut total = 0;
        for c in key.to_lowercase().chars() {
            total += match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' => 2,
                ' ' => 0,
                _ => 1,
            }
        }
        total % 10
    }

    pub fn scrabble(key: &str) -> usize {
        let mut total = 0;
        for c in key.to_lowercase().chars() {
            total += match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 's' | 't' | 'r' => 1,
                'd' | 'g' => 2,
                'b' | 'c' | 'm' | 'p' => 3,
                'f' | 'h' | 'v' | 'w' | 'y' => 4,
                'k' => 5,
                'j' | 'x' => 8,
                'q' | 'z' => 10,
                _ => 0,
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
                detect_collisions: false,
            }
        }
        pub fn toggle_collisions(&mut self) {
            self.detect_collisions = !self.detect_collisions
        }
        pub fn add(&mut self, keystr: &str) -> InsertResult {
            let key = (self.hash_function)(keystr);
            if self.items[key].is_none() {
                self.items[key] = Some(str!(keystr));
                InsertResult::Success
            } else if self.detect_collisions {
                InsertResult::CollisionDetected
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

pub mod karkat {
    use super::*;
    pub struct Vault {
        item: String,
        pub code: u16,
    }
    pub struct Modus {
        pub vaults: Vec<Vault>,
    }

    impl Vault {
        pub fn new(item: &str) -> Self {
            Self {
                item: String::from(item),
                code: Vault::checksum(item),
            }
        }
        pub fn checksum(s: &str) -> u16 {
            let mut sum: u16 = 0;
            for c in s.chars() {
                let mut buffer: [u16; 2] = [0, 0];
                c.encode_utf16(&mut buffer);
                sum = (sum >> 1) + ((sum & 1) << 15);
                sum = sum.wrapping_add(buffer[0] ^ buffer[1]);
            }
            sum
        }
    }
    impl Modus {
        pub fn new() -> Modus {
            Modus { vaults: vec![] }
        }
        pub fn add(&mut self, name: &str) -> InsertResult {
            self.vaults.push(Vault::new(name));
            InsertResult::Success
        }
        pub fn size(&self) -> usize {
            self.vaults.len()
        }
        pub fn from_arr(arr: [u16; 4]) -> u16 {
            let mut r = 0;
            r += (arr[3] & 0x000F) << 0;
            r += (arr[2] & 0x000F) << 4;
            r += (arr[1] & 0x000F) << 8;
            r += (arr[0] & 0x000F) << 12;
            r
        }
        pub fn test(&self, index: usize, key: u16) -> [bool; 4] {
            let mut rt: [bool; 4] = [false, false, false, false];
            if let Some(item) = self.vaults.get(index) {
                let code = item.code;
                for i in 0..4 {
                    let mask = 0xF000 >> (i * 4);
                    let c = code & mask;
                    let k = key & mask;
                    rt[i] = c == k;
                }
                rt
            } else {
                rt
            }
        }
        pub fn take(&mut self, index: usize, key: u16) -> FetchResult {
            if let Some(item) = self.vaults.get(index) {
                if item.code == key {
                    FetchResult::Success(self.vaults.remove(index).item)
                } else {
                    FetchResult::Empty
                }
            } else {
                FetchResult::Empty
            }
        }
    }

    impl fmt::Display for Modus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.vaults.len() {
                write!(f, " |{}|", i)?;
            }
            write!(f, "")
        }
    }
}

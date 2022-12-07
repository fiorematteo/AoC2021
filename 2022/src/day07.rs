use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Rc<RefCell<Directory>> {
    let mut active_directory = Rc::new(RefCell::new(Directory {
        children: HashMap::new(),
        name: "/".to_string(),
        parent: None,
    }));
    let root = active_directory.clone();

    for line in input.lines() {
        if line.starts_with("$") {
            //handle command
            let line = line[2..].trim();
            if let "ls" = line {
                continue;
            }

            let (_, arg) = line.split_once(" ").unwrap();

            // cmd = cd
            if arg == "/" {
                // go to root
                active_directory = root.clone();
                continue;
            }

            if arg == ".." {
                //go to parent
                let parent = active_directory
                    .borrow()
                    .parent
                    .clone()
                    .unwrap_or(active_directory.clone());
                active_directory = parent;
                continue;
            }

            let d = active_directory.borrow();
            if let Child::Directory(dir) = d.children.get(arg).unwrap() {
                let dir = dir.clone();
                drop(d);
                active_directory = dir;
                continue;
            }
            unreachable!();
        }
        if line.starts_with("dir") {
            //handle directory
            let (_, name) = line.split_once(" ").unwrap();
            let directory = Directory {
                children: HashMap::new(),
                name: name.to_string(),
                parent: Some(active_directory.clone()),
            };
            active_directory.borrow_mut().children.insert(
                name.to_string(),
                Child::Directory(Rc::new(RefCell::new(directory))),
            );
            continue;
        }
        //handle file
        let (size, name) = line.split_once(" ").unwrap();
        let file = File {
            name: name.to_string(),
            size: size.parse().unwrap(),
        };
        active_directory
            .borrow_mut()
            .children
            .insert(name.to_string(), Child::File(file));
    }
    root
}

#[aoc(day7, part1)]
pub fn part1(root: &Rc<RefCell<Directory>>) -> usize {
    let mut directories = Vec::new();
    let r = root.borrow();
    r.collect_directories(&mut directories);
    directories
        .iter()
        .map(|d| d.size())
        .filter(|&s| s <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(root: &Rc<RefCell<Directory>>) -> usize {
    let mut directories = Vec::new();
    let r = root.borrow();
    r.collect_directories(&mut directories);

    let total_available = 70_000_000 - r.size();
    let to_delete = 30_000_000 - total_available;
    directories
        .iter()
        .map(|d| d.size())
        .filter(|&s| s >= to_delete)
        .min()
        .unwrap()
}

#[derive(Clone, Default)]
pub struct Directory {
    pub children: HashMap<String, Child>,
    pub name: String,
    pub parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn size(&self) -> usize {
        let mut size = 0;
        for child in self.children.values() {
            match child {
                Child::Directory(dir) => size += dir.borrow().size(),
                Child::File(file) => size += file.size,
            }
        }
        size
    }
}

impl Directory {
    pub fn collect_directories(&self, v: &mut Vec<Directory>) {
        v.push(self.clone());
        for child in self.children.values() {
            match child {
                Child::Directory(dir) => {
                    dir.borrow().collect_directories(v);
                }
                Child::File(_) => {}
            }
        }
    }
}

impl Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Directory {{ name: {}, children: {:#?}",
            self.name, self.children
        )
    }
}

#[derive(Clone)]
pub struct File {
    pub size: usize,
    pub name: String,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {{ name: {}, size: {} }}", self.name, self.size)
    }
}

#[derive(Clone, Debug)]
pub enum Child {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

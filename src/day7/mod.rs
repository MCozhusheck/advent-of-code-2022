use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Eq, PartialEq)]

struct File {
    size: u32,
    name: String,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Directory {
    size: RefCell<u32>,
    name: String,
    parent: Option<Rc<Directory>>,
    sub_dir: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
    pub fn get_size(&self) -> u32 {
        *self.size.borrow()
            + self
                .sub_dir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

pub fn parse(input: String) -> Rc<Directory> {
    let default = Directory {
        size: RefCell::new(0),
        name: "/".to_owned(),
        parent: None,
        sub_dir: RefCell::new(HashMap::new()),
    };
    let root = Rc::new(default);
    let mut cwd = Rc::clone(&root);
    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dir => {
                    let new_dir = cwd.sub_dir.borrow().get(dir).unwrap().clone();
                    cwd = new_dir;
                }
            },
            ("dir", dir_name) => {
                cwd.sub_dir.borrow_mut().insert(
                    dir_name.to_owned(),
                    Rc::new(Directory {
                        size: RefCell::new(0),
                        name: dir_name.to_owned(),
                        parent: Some(Rc::clone(&cwd)),
                        sub_dir: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _) => {
                *cwd.size.borrow_mut() += size.parse::<u32>().unwrap();
            }
        }
    });
    return root;
}

pub fn part1(input: String) -> u32 {
    let root = parse(input);
    let mut to_visit = vec![Rc::clone(&root)];
    let mut total_size = 0;

    while let Some(dir) = to_visit.pop() {
        for d in dir.sub_dir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        if dir.get_size() <= 100000 {
            total_size += dir.get_size()
        };
    }

    return total_size;
}

pub fn part2(input: String) -> u32 {
    let root = parse(input);
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_required = 30000000 - free_space;

    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = u32::MAX;

    while let Some(dir) = to_visit.pop() {
        for d in dir.sub_dir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        if dir.get_size() >= space_required {
            best = best.min(dir.get_size())
        };
    }

    return best;
}

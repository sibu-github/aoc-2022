use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct FolderName(String);

impl FolderName {
    fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

#[derive(Debug, Clone)]
struct DataFile {
    name: String,
    size: usize,
}

impl DataFile {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }
}

type ChildFolder = Rc<RefCell<Folder>>;
type ParentFolder = Weak<RefCell<Folder>>;

#[derive(Debug, Clone)]
struct Folder {
    name: FolderName,
    parent: Option<ParentFolder>,
    child: Vec<ChildFolder>,
    files: Vec<DataFile>,
}

impl Folder {
    fn create_root() -> Self {
        Self {
            name: FolderName::new("/"),
            parent: None,
            child: vec![],
            files: vec![],
        }
    }

    fn new(name: &str, parent: ParentFolder) -> Self {
        Self {
            name: FolderName::new(name),
            parent: Some(parent),
            child: vec![],
            files: vec![],
        }
    }

    fn has_child(&self, dir_name: &str) -> bool {
        self.child
            .iter()
            .any(|c| c.borrow().name.0.as_str() == dir_name)
    }

    fn get_child(&self, dir_name: &str) -> Option<ChildFolder> {
        self.child
            .iter()
            .find(|c| c.borrow().name.0.as_str() == dir_name)
            .cloned()
    }

    fn create_child(&mut self, dir_name: &str, parent: ParentFolder) {
        if !self.has_child(dir_name) {
            let f = Folder::new(dir_name, parent);
            let f = Rc::new(RefCell::new(f));
            self.child.push(f);
        }
    }

    fn has_file(&self, file_name: &str) -> bool {
        self.files.iter().any(|f| f.name.as_str() == file_name)
    }

    fn create_file(&mut self, file_name: &str, file_size: usize) {
        if !self.has_file(file_name) {
            let f = DataFile::new(file_name, file_size);
            self.files.push(f);
        }
    }

    fn size(&self) -> usize {
        let child_size = self.child.iter().map(|c| c.borrow().size()).sum::<usize>();
        let file_size = self.files.iter().map(|f| f.size).sum::<usize>();
        child_size + file_size
    }
}

struct CurrentFolder(Option<ChildFolder>);

impl CurrentFolder {
    fn go_to_root(&mut self, root: ChildFolder) {
        self.0 = Some(root);
    }

    fn go_to_parent(&mut self) {
        let ptr = self.0.clone().unwrap();
        let f = ptr.borrow();
        let parent = f.parent.as_ref().unwrap();
        let parent = parent.upgrade().unwrap();
        self.0 = Some(parent);
    }

    fn go_to_child(&mut self, dir_name: &str) {
        let ptr = self.0.clone().unwrap();
        self.0 = ptr.borrow().get_child(dir_name);
    }

    fn create_dir(&self, dir_name: &str) {
        let pointer = self.0.as_ref().unwrap();
        let parent = Rc::downgrade(pointer);
        pointer.borrow_mut().create_child(dir_name, parent);
    }

    fn create_file(&self, file_name: &str, size: usize) {
        self.0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .create_file(file_name, size);
    }
}

fn list_all_directories(root: ChildFolder) -> Vec<ChildFolder> {
    let mut unvisited = vec![root];
    let mut all_dir = vec![];
    loop {
        let d = unvisited.pop();
        if d.is_none() {
            break;
        }
        let d = d.unwrap();
        all_dir.push(d.clone());
        for child in d.borrow().child.iter() {
            unvisited.push(child.clone());
        }
    }
    all_dir
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let root = Folder::create_root();
    let root = Rc::new(RefCell::new(root));
    let mut curr_folder = CurrentFolder(None);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('$') {
            let line = line.strip_prefix('$').unwrap().trim();
            if line.starts_with("cd") {
                let line = line.strip_prefix("cd").unwrap().trim();
                if line == "/" {
                    curr_folder.go_to_root(root.clone());
                } else if line == ".." {
                    curr_folder.go_to_parent();
                } else {
                    curr_folder.go_to_child(line);
                }
            }
        } else if line.starts_with("dir") {
            let line = line.strip_prefix("dir").unwrap().trim();
            curr_folder.create_dir(line);
        } else {
            let (size, file_name) = line.trim().split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            curr_folder.create_file(file_name, size);
        }
    }
    let all_dir = list_all_directories(root.clone());
    let total_size = all_dir
        .iter()
        .map(|dir| dir.borrow().size())
        .filter(|&size| size <= 100_000)
        .sum::<usize>();
    println!("Total size: {}", total_size);
    let root_size = root.borrow().size();
    let free_space = 70_000_000 - root_size;
    let required_space = 30_000_000 - free_space;
    let mut delete_candidates = all_dir
        .iter()
        .filter(|dir| dir.borrow().size() >= required_space)
        .collect::<Vec<_>>();
    delete_candidates.sort_unstable_by(|a, b| a.borrow().size().cmp(&b.borrow().size()));
    println!(
        "dir name: {}, size: {}",
        delete_candidates[0].borrow().name.0,
        delete_candidates[0].borrow().size()
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_has_child() {
        let root = Folder::create_root();
        let root = Rc::new(RefCell::new(root));
        let dir_c = Folder::new("c", Rc::downgrade(&root));
        let dir_c = Rc::new(RefCell::new(dir_c));
        dir_c.borrow_mut().create_child("a", Rc::downgrade(&dir_c));
        dir_c.borrow_mut().create_child("b", Rc::downgrade(&dir_c));
        assert_eq!(dir_c.borrow().has_child("a"), true);
        assert_eq!(dir_c.borrow().has_child("b"), true);
        assert_eq!(dir_c.borrow().has_child("x"), false);
    }

    #[test]
    fn test_size() {
        let root = Folder::create_root();
        let root = Rc::new(RefCell::new(root));
        root.borrow_mut().create_child("a", Rc::downgrade(&root));
        root.borrow_mut().create_child("b", Rc::downgrade(&root));
        let dir_a = root.borrow().get_child("a").unwrap();
        dir_a.borrow_mut().create_file("a1.txt", 10);
        dir_a.borrow_mut().create_file("a2.txt", 12);
        let dir_b = root.borrow().get_child("b").unwrap();
        dir_b.borrow_mut().create_file("b1.txt", 32);
        dir_b.borrow_mut().create_file("b2.txt", 27);
        assert_eq!(root.borrow().size(), 81);
    }
}

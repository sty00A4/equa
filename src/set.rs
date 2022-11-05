#[derive(Clone)]
pub struct Set<T> where T: PartialEq {
    pub values: Vec<T>
}
impl<T> Set<T> where T: PartialEq {
    pub fn new() -> Self { Self { values: vec![] } }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn add(&mut self, x: T) {
        if !self.values.contains(&x) {
            self.values.push(x);
        }
    }
    pub fn remove(&mut self, x: &T) -> Option<T> {
        for i in 0..self.values.len() {
            if &self.values[i] == x {
                return Some(self.values.remove(i))
            }
        }
        None
    }
    pub fn contains(&self, x: &T) -> bool {
        self.values.contains(x)
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn clear(&mut self) {
        self.values.clear()
    }
}
impl<T> PartialEq for Set<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        for v in self.values.iter() {
            if !other.contains(v) { return false }
        }
        true
    }
}
impl<T> std::fmt::Debug for Set<T> where T: PartialEq, T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f ,"Set{:?}", self.values)
    }
}
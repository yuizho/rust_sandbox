fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    println!("{:?}", v);
    v.push("Budgerigar".to_string());
    println!("{:?}", v);
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));

    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string()); // iterはもう生存していないのでpush可
}

#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

// トレイと境界としてDefaultを設定する
impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        // 戻り値のusize型はCopyトレイとを実装しているので、
        // 所有権のムーブではなく、値がコピーされる
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // 借用(&mut self)経由では所有権は奪えないが、所有権を交換することはできる
            // 第一引数の値を第二引数の値で置き換え、置き換え前の値を返す
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        // T型のデフォルト値を得るために、T型がトレイとを実装している必要がある
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をsize個作る
            .collect::<Vec<_>>()
            .into_boxed_slice() // Box<[T]>に変換
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<T: Default> ToyVec<T> {
    // selfとIter<T>のライフタイムが同一になった
    // 本当はライフタイムを省略できる
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型(トレイトに関連した型)でnextの戻り値型とライフタイムを指定
    type Item = &'vec T;
    // 戻り値のライフタイムはIter<T>のライフタイムと同様
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

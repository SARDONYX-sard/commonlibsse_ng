use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::re::TESBox::TESBox;

use std_fork::option::UntaggedOption;
use std_fork::zeroable::Zeroable;

/// A node in the singly linked list.
///
/// This structure holds an optional pointer to the value (`item`)
/// and the next node (`next`) in the list. The pointers are managed through
/// `TESBox`, which allocates memory from the custom memory manager.
///
/// # Safety
/// The `item` and `next` are raw pointers that must be safely dropped using `TESBox`
/// to avoid memory leaks. If not dropped correctly, memory leakage can occur.
///
/// # Note
/// This list assumes that nodes with zero-initialized values (`item`) are considered "empty" nodes.
/// You should not use such nodes directly as they might be unsafe unless they are properly initialized.
/// This pattern is a consequence of handling raw pointers and being FFI-compatible.
#[derive(PartialEq)]
#[repr(C)]
pub struct Node<T>
where
    T: Zeroable,
{
    /// The last pushed item of the list.
    item: UntaggedOption<T>,

    /// Pointer to the next node in the list.
    /// # Note
    /// The C++ implementation also defined it as next, so we follow it,
    /// but it is actually the Node of the immediately **preceding pushed item**.
    next: Option<NonNull<Node<T>>>,

    /// The linked list uses raw pointers internally due to ownership issues.
    ///
    /// To begin with, `item` and `next` are heap-allocated data from `MemoryManager`, which is managed as `TESBox`. This marker represents the origin of those raw pointers.
    ///
    /// # Safety
    /// If you don't drop them as `TESBox` at drop time, memory leakage will occur.
    marker: PhantomData<TESBox<T>>,
}
const _: () = assert!(core::mem::size_of::<Node<i32>>() == 0x10);
// const _: () = assert!(core::mem::size_of::<Node<*mut ()>>() == 0x18); // Size with enum tag
const _: () = assert!(core::mem::size_of::<Node<*mut ()>>() == 0x10); // Size without enum tag

impl<T> Node<T>
where
    T: Zeroable,
{
    /// Creates a new node containing the given item and pointing to the given next node.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::Node;
    /// let node = Node::new(10, None);
    /// ```
    #[inline]
    pub const fn new(item: T, next: Option<NonNull<Self>>) -> Self {
        Self { item: UntaggedOption::some(item), next, marker: PhantomData }
    }

    /// Creates an empty node with no value and no next pointer.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::Node;
    /// let empty_node = Node::<i32>::new_empty();
    /// ```
    #[inline]
    const fn new_empty() -> Self {
        Self { item: UntaggedOption::none(), next: None, marker: PhantomData }
    }

    /// Leaks the node onto the heap and returns a `NonNull` pointer to it.(To create a next node)
    ///
    /// This method changes the node's state to be managed by the `TESBox` memory manager.
    ///
    /// # Safety
    /// The returned pointer must be eventually dropped using `TESBox` to avoid a memory leak.
    fn leak(self) -> NonNull<Self> {
        NonNull::from(TESBox::leak(TESBox::new(self)))
    }
}

impl<T> Default for Node<T>
where
    T: Zeroable,
{
    #[inline]
    fn default() -> Self {
        Self::new_empty()
    }
}

/// Single-directional linked list (stack-like).
///
/// This linked list behaves like a stack, where the most recently pushed value is always placed at the head.
/// The previous head is stored on the heap as the "next" value, forming a chain of nodes.
///
/// Diagram of the linked list structure (Head -> Next -> Next...):
///
/// ```txt
/// head -> [value] -> [next] -> [next] -> ... -> None
///        ↑
///      latest push
/// ```
///
/// # Note
/// This linked list assumes that a value is considered uninitialized when it is zero-initialized.
/// Using it without zero initialization is dangerous.
///
/// Ideally, an `Option` should be used to indicate whether a value is initialized, but this risk persists due to the strict memory layout requirements imposed by the FFI (Foreign Function Interface) type.
///
/// # Example
/// ```
/// # use commonlibsse_ng::re::BSTList::BSSimpleList;
/// let mut list = BSSimpleList::new();
/// list.push_front(10);
/// list.push_front(20);
/// list.push_front(30);
/// list.push_front(40);
///
/// list.print_tree();
/// assert_eq!(list.len(), 4);
///
/// let mut iter = list.iter();
/// assert_eq!(iter.next(), Some(&40));
/// assert_eq!(iter.next(), Some(&30));
/// assert_eq!(iter.next(), Some(&20));
/// assert_eq!(iter.next(), Some(&10));
///
/// list.pop_front();
///
/// assert_eq!(list.len(), 3);
/// let mut iter = list.iter();
/// assert_eq!(iter.next(), Some(&30));
/// assert_eq!(iter.next(), Some(&20));
/// assert_eq!(iter.next(), Some(&10));
/// ```
#[repr(C)]
pub struct BSSimpleList<T>
where
    T: Zeroable,
{
    /// The last pushed node of the list.
    list_head: Node<T>,
}

impl<T> BSSimpleList<T>
where
    T: Zeroable,
{
    /// Creates a new, empty list.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let list = BSSimpleList::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { list_head: Node::new_empty() }
    }

    /// Pushes a new value to the front of the list.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::new();
    /// list.push_front(10);
    /// ```
    #[inline]
    pub fn push_front(&mut self, value: T) {
        if let Some(current_head) = self.list_head.item.take() {
            let prev_node = Node::new(current_head, self.list_head.next).leak();
            let new_node = Node::new(value, Some(prev_node));
            self.list_head = new_node;
        } else {
            self.list_head.item = UntaggedOption::some(value);
        }
    }

    /// Removes and returns the first element in the list.
    ///
    /// # Returns
    /// `Some(TESBox<Node<T>>)` if the list is not empty.
    /// `None` if the list is empty.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::new();
    /// list.push_front(1);
    /// assert!(list.pop_front().is_some());
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = &mut self.list_head;
        let last_item = head.item.take();

        if let Some(next_node) = head.next {
            let mut next_node = unsafe { TESBox::from_non_null(next_node) };
            head.item = next_node.as_mut().item.take().into();
            head.next = next_node.next;
            drop(next_node);
        };

        last_item
    }

    /// Returns a reference to the front element.
    ///
    /// # Returns
    /// `None` if the list is empty.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let list = BSSimpleList::<i32>::new();
    /// assert!(list.front().is_none());
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.list_head.next.as_ref().and_then(|node| unsafe { node.as_ref().item.as_ref() })
    }

    /// Returns `true` if the list is empty.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.list_head.next.is_none()
    }

    /// Returns the number of elements in the list.
    ///
    /// Note that the computation cost is `O(n)` since it is a linked list.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::new();
    /// list.push_front(1); // 2
    /// list.push_front(2); // 1
    /// list.push_front(3); // 0
    ///
    /// assert_eq!(list.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        if self.list_head.item.is_none() {
            return 0;
        }

        let mut len = 1; // count the first node.
        let mut current = self.list_head.next.as_ref();
        while let Some(node) = current {
            len += 1;
            current = unsafe { node.as_ref().next.as_ref() };
        }
        len
    }

    /// Inserts a new value after the given node.
    ///
    /// # Returns
    /// A mutable reference to the inserted node, or `None` if insertion failed.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::{BSSimpleList, Node};
    /// let mut list = BSSimpleList::new();
    /// let mut first = Node::new(1, None);
    /// list.insert_after(&mut first, 2);
    /// ```
    pub fn insert_after(&mut self, pos: &mut Node<T>, value: T) -> Option<&mut Node<T>> {
        pos.next = Some(Node::new(value, pos.next.take()).leak());
        pos.next.as_mut().map(|n| unsafe { n.as_mut() })
    }

    /// Removes the node after the given position.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::BSTList::{BSSimpleList, Node};
    ///
    /// let mut list = BSSimpleList::new();
    /// let mut first = Node::new(1, None);
    /// list.insert_after(&mut first, 2);
    /// list.erase_after(&mut first);
    /// ```
    #[inline]
    pub const fn erase_after(&mut self, pos: &mut Node<T>) {
        if let Some(mut node) = pos.next.take() {
            pos.next = unsafe { node.as_mut().next.take() };
        }
    }

    /// Returns a reference to the node at the given position.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::new();
    /// list.push_front(1); // 3
    /// list.push_front(2); // 2
    /// list.push_front(3); // 1
    /// list.push_front(4); // 0
    ///
    /// assert_eq!(list.get(1), Some(&3));
    /// ```
    #[inline]
    pub fn get(&self, pos: usize) -> Option<&T> {
        for (idx, value) in self.iter().enumerate() {
            if idx == pos {
                return Some(value);
            }
        }

        None
    }

    /// Returns a mutable reference to the node at the given position.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::new();
    /// list.push_front(1); // 3
    /// list.push_front(2); // 2
    /// list.push_front(3); // 1
    /// list.push_front(4); // 0
    ///
    /// if let Some(value) = list.get_mut(2) {
    ///    *value = 5;
    /// }
    /// assert_eq!(list.get(2), Some(&5));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, pos: usize) -> Option<&mut T> {
        for (idx, value) in self.iter_mut().enumerate() {
            if idx == pos {
                return Some(value);
            }
        }

        None
    }

    /// Clears the entire list.
    #[inline]
    pub fn clear(&mut self) {
        let mut current = self.list_head.next.take();

        self.list_head.item = UntaggedOption::none();

        while let Some(node_ptr) = current {
            unsafe {
                let node = node_ptr.as_ref();
                current = node.next;
                drop(TESBox::from_raw(node_ptr.as_ptr()));
            }
        }
    }

    /// Returns an iterator over the list.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            fist_node_item: self.list_head.item.as_ref(),
            current: self.list_head.next,
            _marker: PhantomData,
        }
    }

    /// Returns a mutable iterator over the list.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            fist_node_item: self.list_head.item.as_mut(),
            current: self.list_head.next,
            _marker: PhantomData,
        }
    }

    /// Consumes the `SimpleList` and returns a `Vec<T>` containing all elements in order.
    ///
    /// This method traverses the linked list and collects the values into a `Vec<T>`.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::new();
        for node in self {
            vec.push(node);
        }
        vec
    }

    /// Resizes the list to the given length.
    ///
    /// - If the list is shorter, it will be extended with the given value.
    /// - If the list is longer, it will be truncated.
    ///
    /// # Example
    /// - If length is 0, nothing is done.
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::<i32>::new();
    /// list.resize(3, 0);
    /// assert_eq!(list.len(), 0);
    /// ```
    ///
    /// - If the list is longer, it will be truncated.
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// list.push_front(4);
    /// list.resize(3, 0);
    /// assert_eq!(list.len(), 3);
    /// ```
    ///
    /// - If the list is shorter, it will be extended with the given value.
    /// ```
    /// # use commonlibsse_ng::re::BSTList::BSSimpleList;
    /// let mut list = BSSimpleList::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.resize(4, 5);
    ///
    /// assert_eq!(list.len(), 4);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// ```
    #[inline]
    pub fn resize(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        let this = &mut *self;
        let current_len = this.len();

        match new_size {
            count if count < current_len => this.truncate(count),
            count if count > current_len => this.grow(count - current_len, value),
            _ => {}
        }
    }

    #[inline]
    fn grow(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        for _ in 0..count {
            self.push_front(value.clone());
        }
    }

    #[inline]
    fn truncate(&mut self, count: usize) {
        while self.len() > count {
            self.pop_front();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T>
where
    T: Zeroable,
{
    fist_node_item: Option<&'a T>,
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Zeroable,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.fist_node_item.take() {
            return Some(item);
        }

        self.current.take().and_then(|node_ptr| {
            let node_ref = unsafe { node_ptr.as_ref() };
            self.current = node_ref.next;
            node_ref.item.as_ref()
        })
    }
}

pub struct IterMut<'a, T>
where
    T: Zeroable,
{
    fist_node_item: Option<&'a mut T>,
    /// next node item
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Zeroable,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.fist_node_item.take() {
            return Some(item);
        }

        self.current.take().and_then(|mut node_ptr| {
            let node_ref = unsafe { node_ptr.as_mut() };
            self.current = node_ref.next;
            node_ref.item.as_mut()
        })
    }
}

/// An owning iterator over the elements of a `BSSimpleList<T>`.
///
/// This iterator consumes the list and yields each element by value.
pub struct IntoIter<T>
where
    T: Zeroable,
{
    fist_node_item: Option<T>,
    current: Option<NonNull<Node<T>>>,
}

impl<T> Iterator for IntoIter<T>
where
    T: Zeroable,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.fist_node_item.take() {
            return Some(item);
        }

        self.current.take().and_then(|node_ptr| {
            let mut boxed = unsafe { TESBox::from_non_null(node_ptr) };
            self.current = boxed.next;
            boxed.item.take()
        })
    }
}

impl<T> IntoIterator for BSSimpleList<T>
where
    T: Zeroable,
{
    type Item = T;

    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let mut this = self; // To avoid mutable error.
        IntoIter { fist_node_item: this.list_head.item.take(), current: this.list_head.next.take() }
    }
}

impl<'a, T> IntoIterator for &'a BSSimpleList<T>
where
    T: Zeroable,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut BSSimpleList<T>
where
    T: Zeroable,
{
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Extend<T> for &mut BSSimpleList<T>
where
    T: Zeroable,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_front(item);
        }
    }
}

impl<T> Extend<T> for BSSimpleList<T>
where
    T: Zeroable,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_front(item);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Drop for BSSimpleList<T>
where
    T: Zeroable,
{
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> fmt::Debug for BSSimpleList<T>
where
    T: fmt::Debug + Zeroable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for BSSimpleList<T>
where
    T: Zeroable,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for BSSimpleList<T>
where
    T: Clone + Zeroable,
{
    #[inline]
    fn clone(&self) -> Self {
        let mut ret = Self::new();

        let mut current = self.list_head.next.as_ref();
        while let Some(node) = current {
            let node = unsafe { node.as_ref() };
            if let Some(item) = node.item.as_ref() {
                ret.push_front(item.clone());
            }
            current = node.next.as_ref();
        }

        ret
    }
}

impl<T> PartialEq for BSSimpleList<T>
where
    T: PartialEq + Zeroable,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.iter();
        let mut b = other.iter();
        loop {
            match (a.next(), b.next()) {
                (Some(x), Some(y)) if x == y => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
impl<T> Eq for BSSimpleList<T> where T: Eq + Zeroable {}

impl<T> PartialOrd for BSSimpleList<T>
where
    T: PartialOrd + Zeroable,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut a = self.iter();
        let mut b = other.iter();
        loop {
            match (a.next(), b.next()) {
                (Some(x), Some(y)) => match x.partial_cmp(y) {
                    Some(Ordering::Equal) => {}
                    non_eq => return non_eq,
                },
                (None, None) => return Some(Ordering::Equal),
                (None, _) => return Some(Ordering::Less),
                (_, None) => return Some(Ordering::Greater),
            }
        }
    }
}

impl<T> Ord for BSSimpleList<T>
where
    T: Ord + Zeroable,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let mut a = self.iter();
        let mut b = other.iter();
        loop {
            match (a.next(), b.next()) {
                (Some(x), Some(y)) => match x.cmp(y) {
                    Ordering::Equal => {}
                    non_eq => return non_eq,
                },
                (None, None) => return Ordering::Equal,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
            }
        }
    }
}

impl<T> Hash for BSSimpleList<T>
where
    T: Hash + Zeroable,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.iter() {
            item.hash(state);
        }
    }
}

impl<T> BSSimpleList<T>
where
    T: fmt::Display + Zeroable,
{
    /// Prints the list as a tree-like structure
    pub fn print_tree(&self) {
        /// Recursively prints each node in a tree-like structure
        fn print_node<T>(node: &Node<T>, depth: usize)
        where
            T: fmt::Display + Zeroable,
        {
            if let Some(item) = node.item.as_ref() {
                let indentation = " ".repeat(depth * 2);
                println!("{indentation}- {item}");
                if let Some(next_node) = &node.next {
                    // If there's a next node, print it recursively
                    unsafe {
                        let next_node_ref = next_node.as_ref();
                        print_node(next_node_ref, depth + 1);
                    }
                }
            }
        }

        print_node(&self.list_head, 0);
    }
}

#[cfg(feature = "test_on_ci")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bs_simple_list() {
        let mut list = BSSimpleList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);
        list.push_front(40);

        list.print_tree();
        assert_eq!(list.len(), 4);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&40));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&10));

        list.pop_front();

        assert_eq!(list.len(), 3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&10));
    }

    #[test]
    fn test_resize() {
        let mut list = BSSimpleList::<i32>::new();
        list.resize(3, 0);
        assert_eq!(list.len(), 0);

        let mut list = BSSimpleList::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.resize(3, 0);
        assert_eq!(list.len(), 3);
    }
}

use std::marker::PhantomData;
use std::ptr::NonNull;

/// LinkedList
#[derive(Debug, Clone)]
#[repr(C)]
pub struct BSSimpleList<T>
where
    T: Clone,
{
    list_head: Node<T>,
}

impl<T> Default for BSSimpleList<T>
where
    T: Clone,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Node<T> {
    item: NonNull<T>,
    next: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<T>>,
}

impl<T> Default for Node<T> {
    #[inline]
    fn default() -> Self {
        Self { item: NonNull::dangling(), next: None, marker: PhantomData }
    }
}

impl<T> Node<T> {
    #[inline]
    fn new(item: T, next: Option<NonNull<Self>>) -> Self {
        let item_ptr = NonNull::from(Box::leak(Box::new(item)));
        Self { item: item_ptr, next, marker: PhantomData }
    }
}

impl<T> BSSimpleList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { list_head: Node::default() }
    }

    /// Adds the given node to the front of the list.
    ///
    /// # Safety
    /// `node` must point to a valid node that was boxed and leaked using the list's allocator.
    /// This method takes ownership of the node, so the pointer should not be used again.
    #[inline]
    unsafe fn push_front_node(&mut self, value: NonNull<Node<T>>) {
        self.list_head.next = Some(value);
    }

    #[inline]
    pub fn push_front(&mut self, elt: T) {
        let node = Box::new(Node::new(elt, None));
        let node_ptr = NonNull::from(Box::leak(node));
        // SAFETY: node_ptr is a unique pointer to a node we boxed with self.alloc and leaked
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    pub fn pop_front(&mut self) -> Option<Box<Node<T>>> {
        let next_node = self.list_head.next.take()?;
        unsafe {
            let next_node_ref = next_node.as_ref();
            self.list_head.item = next_node_ref.item;
            self.list_head.next = next_node_ref.next;
            Some(Box::from_raw(next_node.as_ptr())) // Drop the old head
        }
    }

    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.list_head.next.as_ref().map(|node| unsafe { node.as_ref().item.as_ref() })
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.list_head.next.is_none()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        let mut len = 0;
        let mut current = self.list_head.next.as_ref();
        while let Some(node) = current {
            len += 1;
            current = unsafe { node.as_ref().next.as_ref() };
        }
        len
    }

    #[inline]
    pub fn resize(&mut self, count: usize) {
        let this = &mut *self;
        let value = unsafe { NonNull::new_unchecked(Box::into_raw(Box::default())) };
        let current_len = this.len();

        match count {
            count if count < current_len => this.truncate(count),
            count if count > current_len => this.grow(count - current_len, value),
            _ => {}
        }
    }

    #[inline]
    fn grow(&mut self, count: usize, value: NonNull<Node<T>>) {
        for _ in 0..count {
            unsafe { self.push_front_node(value) };
        }
    }

    #[inline]
    fn truncate(&mut self, count: usize) {
        while self.len() > count {
            self.pop_front();
        }
    }

    pub fn insert_after(&mut self, pos: &mut Node<T>, value: T) -> Option<&mut Node<T>> {
        let new_node = Box::new(Node::new(value, pos.next.take()));

        pos.next = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
        pos.next.as_mut().map(|n| unsafe { n.as_mut() })
    }

    #[inline]
    pub fn erase_after(&mut self, pos: &mut Node<T>) {
        if let Some(mut node) = pos.next.take() {
            pos.next = unsafe { node.as_mut().next.take() };
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        let mut current = self.list_head.next.take();
        while let Some(mut node) = current {
            current = unsafe { node.as_mut().next.take() };
        }
    }

    pub fn copy_from(&mut self, other: &Self) {
        let mut current = other.list_head.next.as_ref();
        while let Some(node) = current {
            let node = unsafe { node.as_ref() };
            unsafe { self.push_front(node.item.as_ref().clone()) };
            current = node.next.as_ref();
        }
    }

    #[inline]
    pub const fn iter(&self) -> Iter<T> {
        Iter { current: self.list_head.next, _marker: PhantomData }
    }
}

pub struct Iter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node_ptr| {
            let node_ref = unsafe { node_ptr.as_ref() };
            self.current = node_ref.next;
            unsafe { node_ref.item.as_ref() }
        })
    }
}

impl<T> Drop for BSSimpleList<T>
where
    T: Clone,
{
    #[inline]
    fn drop(&mut self) {
        while self.list_head.next.is_some() {
            self.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bssimplelist() {
        let mut list: BSSimpleList<i32> = BSSimpleList::new();
        list.push_front(10);
        list.push_front(20);

        for value in list.iter() {
            println!("{}", value);
        }

        list.pop_front();
        println!("After pop:");
        for value in list.iter() {
            println!("{}", value);
        }
    }
}

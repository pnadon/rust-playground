// https://www.interviewcake.com/concept/java/heapsort
// My housemate challenged me to make this, and so I complied.
pub trait HeapSort<T: PartialOrd> {
  fn heap_sort(&mut self);
}

impl<T: PartialOrd> HeapSort<T> for [T] {
  fn heap_sort(&mut self) {
    build_max_heap(self);
    for slice in (1..=self.len()).rev() {
      heapify(&mut self[0..slice]);
    }
  }
}

// depth vs len: 1:1, 2:2, 3:4, 4:8, 5:16... log_2(len)
fn build_max_heap<T>(vec: &mut [T]) 
where T: PartialOrd
{
  let max_depth = log2(vec.len());
  for depth in (0..(max_depth - 1)).rev() {
    for offset in 0..(1 << depth) {
      bubble_down(vec, heap_index(depth, offset));
    }
  }
}

fn heap_index(depth: usize, offset: usize) -> usize {
  (1 << depth) - 1 + offset 
}

fn bubble_down<T>(vec: &mut [T], index: usize)
where T: PartialOrd
{
  let left_child = index * 2 + 1;
  let right_child = index * 2 + 2;

  if left_child >= vec.len() {
    return;
  }

  let largest_child = if right_child >= vec.len() {
    left_child
  } else if vec[left_child] > vec[right_child] {
    left_child
  } else {
    right_child
  };

  if vec[largest_child] > vec[index] {
    vec.swap(largest_child, index);
    bubble_down(vec, largest_child);
  }
}

fn log2(mut num: usize) -> usize {
  let mut ans = 0;
  while num > 0 {
    num >>= 1;
    ans += 1;
  }
  ans
}

fn heapify<T>(vec: &mut [T]) 
where T: PartialOrd
{
  let last_idx = vec.len() - 1;
  vec.swap(0, last_idx);
  bubble_down(&mut vec[..last_idx], 0);
}

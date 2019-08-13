// adpated from post here by Matthieu M.
// https://stackoverflow.com/questions/42134874/are-there-equivalents-to-slicechunks-windows-for-iterators-to-loop-over-pairs

pub struct Chunks<I: Iterator> {
    elements: Vec<<I as Iterator>::Item>,
    underlying: I,
}

impl<I: Iterator> Chunks<I> {
    pub fn new(iterator: I, size: usize) -> Chunks<I> {
        // for slice in Chunks::new(BufReader::new(io::stdin()).lines(), 2) {
        assert!(size > 0);

        let mut result = Chunks {
           underlying: iterator, elements: Vec::with_capacity(size)
        };
        result.refill(size);
        result
    }

    fn refill(&mut self, size: usize) {
        assert!(self.elements.is_empty());

        for _ in 0..size {
            match self.underlying.next() {
                Some(item) => self.elements.push(item),
                None => break,
            }
        }
    }
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None;
        }

        let new_elements = Vec::with_capacity(self.elements.len());
        let result = std::mem::replace(&mut self.elements, new_elements);

        self.refill(result.len());

        Some(result)
    }
}

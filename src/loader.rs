use hound;

struct Slides<'a, T: 'a> {
    v: &'a [T],
    chunk_size: usize,
    step_size: usize,
}

impl<'a, T> Iterator for Slides<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        if self.chunk_size > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.chunk_size]);
            self.v = &self.v[self.step_size..];
            ret
        }
    }
}

fn slides<T>(slice: &Vec<T>, chunk_size: usize, step_size: usize) -> Slides<T> {
    assert!(chunk_size != 0);
    assert!(step_size != 0);
    assert!(chunk_size >= step_size);
    Slides {
        v: slice,
        chunk_size,
        step_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut reader = hound::WavReader::open("/home/sthomer/Code/CLionProjects/predict/export.wav").unwrap();
        let samples: Vec<i16> = reader.samples().filter_map(Result::ok).collect();
        let slides_1 = slides(&samples, 8, 4);
        let slides_1 = slides_1.collect::<Vec<_>>();
        let slides_2 = slides(&slides_1, 8, 4);
        let slides_2 = slides_2.collect::<Vec<_>>();
        let slides_3 = slides(&slides_2, 8, 4);
        let slides_3 = slides_3.collect::<Vec<_>>();
    }
}

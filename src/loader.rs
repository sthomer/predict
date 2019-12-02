use hound;
use std::fs;
use std::io;

pub fn load_text(path: &String) -> Result<Vec<String>, io::Error>  {
    let text = fs::read_to_string(path)?;
    Ok(text.split_whitespace().map(|s| s.to_string()).collect())
}

pub fn load_wav(path: &String) -> Result<Vec<f64>, hound::Error> {
    Ok(hound::WavReader::open(path)?
        .into_samples()
        .filter_map(Result::ok)
        .map(|sample: i16| sample as f64 / 32768.0) // Assumes wav is i16
        .collect())
}

struct Slides<'a, T: 'a> {
    v: &'a [T],
    window: usize,
    step: usize,
}

impl<'a, T> Iterator for Slides<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        if self.window > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.window]);
            self.v = &self.v[self.step..];
            ret
        }
    }
}

fn slides<T>(slice: &Vec<T>, window: usize, step: usize) -> Slides<T> {
    assert!(window != 0);
    assert!(step != 0);
    assert!(window >= step);
    Slides {
        v: slice,
        window,
        step,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //        let slides_1 = slides(&samples, 8, 4);
        //        let slides_1 = slides_1.collect::<Vec<_>>();
        //        let slides_2 = slides(&slides_1, 8, 4);
        //        let slides_2 = slides_2.collect::<Vec<_>>();
        //        let slides_3 = slides(&slides_2, 8, 4);
        //        let slides_3 = slides_3.collect::<Vec<_>>();
    }
}

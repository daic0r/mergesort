use rand::distributions::{Distribution, Uniform};

pub mod mergesort {
    use crate::mergesort;

    fn merge<T: PartialOrd + Clone>(data: &mut [T], data2: &mut [T]) -> Vec<T> {
        let len = data.len();
        let len2 = data2.len();

        let mut ret = vec![];
        let mut i = 0usize;
        let mut j = 0usize;

        while i < len || j < len2 {
            if j == len2 || (i < len && data[i] <= data2[j]) {
                ret.push(data[i].clone());
                i += 1;
            }
            else 
            if i == len || (j < len2 && data2[j] < data[i]) {
                ret.push(data2[j].clone());
                j += 1;
            }
        }

        assert!(ret.len() == len + len2);
        ret
    }

    fn sort_impl<T: PartialOrd + Clone>(data: &mut [T], left: usize, right: usize) {
        if right-left < 2 {
            return;
        }

        let mid = left + ((right - left) / 2);
        sort_impl(data, left, mid);
        sort_impl(data, mid, right);

        let (slice_left, slice_right) = data[left..right].split_at_mut(mid - left);
        let merged = merge(slice_left, slice_right);
        
        assert_eq!(merged.len(), data[left..right].len());

        let mut dst = data[left..right].iter_mut();
        for src in merged.into_iter() {
            if let Some(d) = dst.next() {
                *d = src;
            }
        }
        //data[left..right].clone_from_slice(&merged);
    }

    pub fn sort<T: PartialOrd + Clone>(data: &mut [T]) {
        sort_impl(data, 0, data.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_merge_sort1() {
        let mut v = vec![12, 12, 23, 4 , 6, 6, 10, -35, 28];

        let l = v.len();
        mergesort::sort(&mut v);

        assert_eq!(v, vec![-35, 4, 6, 6, 10, 12, 12, 23, 28]);
    }

    #[test]
    fn test_merge_empty_vec() {
        let mut v: Vec<i32> = vec![];

        let l = v.len();
        mergesort::sort(&mut v);

        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_merge_one_element() {
        let mut v: Vec<i32> = vec![1];

        let l = v.len();
        mergesort::sort(&mut v);

        assert_eq!(v, vec![1]);
    }

    #[test]
    fn test_merge_two_elements() {
        let mut v: Vec<i32> = vec![10, 1];

        let l = v.len();
        mergesort::sort(&mut v);

        assert_eq!(v, vec![1, 10]);
    }

    #[test]
    fn test_compare_std() {
        let mut v: Vec<i32> = vec![];

        let mut rng = rand::thread_rng();
        let rand = Uniform::from(i32::MIN..i32::MAX);
        for i in 1..4096 {
            v.push(rand.sample(&mut rng));
        }

        let mut v2 = v.clone();

        mergesort::sort(&mut v);

        v2.sort();

        assert_eq!(v, v2);
    }
}

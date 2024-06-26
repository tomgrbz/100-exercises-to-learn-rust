// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    
    let s = thread::scope(|scope| {
        let (l1, r1) = v.split_at(v.len() / 2);
        let s1 = scope.spawn(|| {
            l1.iter().sum::<i32>()
        });
        let sum1 = s1.join().unwrap();
        let s2 = scope.spawn(|| {
            r1.iter().sum::<i32>()
        });
        let sum2 = s2.join().unwrap();
        return sum1 + sum2;
    });
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}

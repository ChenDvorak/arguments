use std::collections::LinkedList;

pub fn add_two_numbers(n1: LinkedList<i32>, n2: LinkedList<i32>) -> LinkedList<i32> {
    let mut result: LinkedList<i32> = LinkedList::new();
    
    let mut num1 = n1.iter();
    let mut num2 = n2.iter();

    let mut r1 = num1.next();
    let mut r2 = num2.next();
    let mut carry = false;
    while r1.is_some() || r2.is_some() {
        let mut sum: i32 = 0;
        if let Some(v) = r1 {
            sum += v;
            r1 = num1.next();
        }
        if let Some(v) = r2 {
            sum += v;
            r2 = num2.next();
        }

        if carry {
            sum += 1;
        }

        result.push_back(sum % 10);

        if sum > 9 {
            carry = true;
        }
        
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_708() {
        let mut n1: LinkedList<i32> = LinkedList::new();
        let mut n2: LinkedList<i32> = LinkedList::new();

        n1.push_back(2);
        n1.push_back(4);
        n1.push_back(3);

        n2.push_back(5);
        n2.push_back(6);
        n2.push_back(4);

        let result = add_two_numbers(n1, n2);

        let mut iter = result.iter();
        if let Some(v) = iter.next() {
            assert_eq!(*v, 7);
        }
        if let Some(v) = iter.next() {
            assert_eq!(*v, 0);
        }
        if let Some(v) = iter.next() {
            assert_eq!(*v, 8);
        }
    }
}
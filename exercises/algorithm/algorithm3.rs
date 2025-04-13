/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn merge<T>(array: &mut [T], mid: usize)
where
    T: std::cmp::PartialOrd + Clone,
{
    let mut tar = vec![];
    let mut i = 0;
    let mut j = mid;
    while i!=mid && j!=array.len() {
        if array[i] < array[j] {
            tar.push(array[i].clone());
            i += 1;
        } else {
            tar.push(array[j].clone());
            j += 1;
        }
    }
     
    while i!=mid {
        tar.push(array[i].clone());
        i += 1;
    }

    while j != array.len() {
        tar.push(array[j].clone());
        j += 1;
    }

    for i in 0..array.len() {
        array[i] = tar[i].clone();
    }
}

fn sort<T>(array: &mut [T])
where
    T: std::cmp::PartialOrd + Clone,
{
	//TODO
    let length = array.len();
    if length > 1 {
        let mid = length / 2 as usize;
        sort(&mut array[0..mid]);
        sort(&mut array[mid..]);
        merge(array, mid);
    }
    


}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
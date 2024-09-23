pub fn array_union_bruteforce() {
    let arr1 = vec![1, 1, 2, 3, 4, 5];
    println!("Arr1: {:?}", arr1);
    let arr2 = vec![2, 3, 4, 4, 5, 6];
    println!("Arr2: {:?}", arr2);

    let mut st: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();

    for i in arr1 {
        st.insert(i);
    }
    for i in arr2 {
        st.insert(i);
    }

    println!("Union: {:?}", st);
}

pub fn array_union_twopointer() {
    let arr1 = vec![1, 1, 2, 3, 4, 5];
    println!("Arr1: {:?}", arr1);
    let arr2 = vec![2, 3, 4, 4, 5, 6];
    println!("Arr2: {:?}", arr2);

    let mut union: Vec<i32> = Vec::new();

    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            let last_ele = union.last();

            match last_ele {
                Some(last) => {
                    if *last != arr1[i] {
                        union.push(arr1[i]);
                    }
                }
                None => {
                    union.push(arr1[i]);
                }
            }
            i += 1;
        } else {
            let last_ele = union.last();

            match last_ele {
                Some(last) => {
                    if *last != arr2[j] {
                        union.push(arr2[j]);
                    }
                }
                None => {
                    union.push(arr2[j]);
                }
            }
            j += 1;
        }
    }

    while i < arr1.len() {
        let last_ele = union.last();

        match last_ele {
            Some(last) => {
                if *last != arr1[i] {
                    union.push(arr1[i]);
                }
            }
            None => {
                union.push(arr1[i]);
            }
        }
        i += 1;
    }

    while j < arr2.len() {
        let last_ele = union.last();

        match last_ele {
            Some(last) => {
                if *last != arr2[j] {
                    union.push(arr2[j]);
                }
            }
            None => {
                union.push(arr2[j]);
            }
        }
        j += 1;
    }

    println!("Union: {:?}", union);
}

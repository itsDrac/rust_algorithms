// Compare two arrays and return a new array with any items only found in one of the two given arrays,
// but not both. In other words, return the symmetric difference of the two arrays.

// Note: You can return the array with its elements in any order.

#[allow(dead_code)]
#[allow(unused_variables)]
fn diff_array<'a>(arr1: Vec<&'a str>, arr2: Vec<&'a str>) -> Vec<&'a str> {
    let mut v3: Vec<&'a str> = Vec::new();
    for search in arr2.iter() {
        if !arr1.iter().any(|s| s==search) {
            v3.push(search);
        }
    }
    for search in arr1.iter() {
        if !arr2.iter().any(|s| s==search) {
            v3.push(search);
        }
    }
    v3.sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr1 = vec![
            "diorite",
            "andesite",
            "grass",
            "dirt",
            "pink wool",
            "dead shrub",
        ];
        let arr2 = vec!["diorite", "andesite", "grass", "dirt", "dead shrub"];

        assert_eq!(diff_array(arr1, arr2), ["pink wool"]);
    }

    #[test]
    fn test2() {
        let arr1 = vec!["andesite", "grass", "dirt", "pink wool", "dead shrub"];
        let arr2 = vec!["diorite", "andesite", "grass", "dirt", "dead shrub"];

        assert_eq!(diff_array(arr1, arr2), ["diorite", "pink wool"]);
    }

    #[test]
    fn test3() {
        let arr1 = vec!["andesite", "grass", "dirt", "dead shrub"];
        let arr2 = vec!["andesite", "grass", "dirt", "dead shrub"];

        assert_eq!(diff_array(arr1, arr2).len(), 0);
    }
}

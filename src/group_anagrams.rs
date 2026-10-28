// GROUP ANAGRAMS 

use std::collections::HashMap;

fn main() {
    let arr = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    let result: Vec<Vec<&str>> = group_anagrams(arr);
    println!("result: {:?}", result);
}

fn group_anagrams(arr: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups: HashMap<[usize; 26], Vec<&str>> = HashMap::new();

    
    for word in arr {
        let mut counts = [0; 26];
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            counts[index] +=1;
        }
        groups.entry(counts).or_default().push(word);
    }

    groups.into_values().collect()
}
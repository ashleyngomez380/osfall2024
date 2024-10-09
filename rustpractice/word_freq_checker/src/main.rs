fn most_frequent_word(text: &str) -> (String, usize) {
    let words:Vec<&str>=text.split_whitespace().collect();
    let mut uniquewords : Vec<String> = Vec ::new(); 
    let mut count :Vec<usize> = Vec :: new();

    for word in words {

        // Make lowercase 
        let lowecase = word.to_lowercase();

        // Check if already found  
        match uniquewords.iter().position(|x| *x == lowecase){
            Some(idex)=> {
                count[idex]+=1;
            }
            None => {
                uniquewords.push(lowecase);
                count.push(1);
            }
        }
    }
    //Initilizing the max_word and max_count
    let mut max_count :usize =0;
    let mut max_word = String::new();

// Looking for the max with iteration
    for (i,count) in count.iter().enumerate(){
        //If we find a new max word
        if *count > max_count{
            //Update the max_count
            max_count = *count;
            //Get the word
            max_word = uniquewords[i].clone();
        }
    }
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
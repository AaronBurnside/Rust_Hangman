use std::io;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    loop{
        println!("Are You Ready to Play(Yes/No): ");
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("Failed to read line");

        if confirm.trim() == "Yes"{
            game();
            break;
        }
        else if confirm.trim() == "No" {
            println!("Good-Bye");
            std::process::exit(0);
        }
        else{
            println!("Please be sure to Input either Yes or No");
            continue
        }
    }
    

}


fn game(){
    println!("Please select from the following options:");
    println!("1: Start a Game using a Random Word from the Word Bank");
    println!("2: Start a Game with a Preselected Word");
    println!("3: Add a New Word to the Word Bank");

    loop {
        
        let mut command = String::new();
        io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if command == 1{
            let mut word = random_word();
            let word = word.trim();
            Hangman(word);
        }
        else if command == 2{
            let mut word = String::new();
            println!("Please Select the Word you would like to Use: ");
            io::stdin().read_line(&mut word).expect("Failed to read line");
            let word = word.trim();
            Hangman(word);
            continue
        }
        else if command == 3{
            let mut word = String::new();
            println!("Please enter the Word you would like to add: ");
            io::stdin().read_line(&mut word).expect("Failed to read line");
            let word = word.trim();
            add_word_to_bank(word);
            println!("Your word {word} has been added to the word bank, please select another command: ");
            continue
        }
        else if command == 4{
            continue
        }
        else{
            println!("Please be sure to Input a value that matches a command");
            continue
        }

    }
}


fn add_word_to_bank(word: &str){
    let mut data_file = OpenOptions::new().append(true).open("Word_Bank.txt").expect("cannot open File");
    data_file.write("\n".as_bytes());
    data_file.write(word.as_bytes());
    return
}

fn file_size() -> i32{
    let mut file = OpenOptions::new().read(true).open("Word_Bank.txt").expect("cannot open File");
    //let file = fs::File::open("Word_Bank");
    let mut reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut size = 0;
    for line in lines{
        size += 1;
    }
    return size;
}

fn random_word() -> String {
    let mut word = String::new();
    let mut size = file_size();

    let number = rand::thread_rng().gen_range(0..=size);
    let mut x = 0;

    //let mut file = File::open("Word_Bank.txt").unwrap();
    let mut file = OpenOptions::new().read(true).open("Word_Bank.txt").expect("cannot open File");
    //let file = match file{
       //Ok(file) => file,
        //Err(error) => panic!("Problem with the Word Bank File: {:?}", error),
   // };
    let mut reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    for line in lines{
        x +=1; 
        if x == number{
            if let Ok(_line) = line{
                word = _line;
            }     
        }
    }
    //let word = fs::read_to_string("Word_Bank.txt");
    let word = word.trim().to_string();
    return word;
}

fn Hangman(word : &str){
    let word_size = word.len();
    let mut found = 0;
    let mut guessedVal = String::new();
    let mut guess_flag = false;
    let mut hit_flag = false;
    println!("The word is {word_size} characters long");
    let mut x = 0;
    let mut word_container = String::new();
    let mut miss: i32 = 0;
    let contain = "_";
    loop{
        x += 1;
        match x.cmp(&word_size) {
            Ordering::Less => word_container.push('_'),
            Ordering::Greater => break,
            Ordering::Equal => {
                break;
            }
        }
    }

    loop {
        Hangman_Draw(miss);
        if miss >= 7{
            println!("You have lossed");
            break;
        }
        println!("{word_container}"); 
        println!("Please input the letter you would like to guess.");
        
        let mut  guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();
        //let mut check = guess.trim().to_string();
        for i in guessedVal.chars(){
            let g = guess.trim().chars().nth(0).unwrap();
            if g == i{
                guess_flag = true;
            }
        }
        if guess_flag == true{
            guess_flag = false;
        }
        else{
            guessedVal.push_str(guess);
            let mut x = 0;
            for c in word.chars() {
                x += 1;
                let g = guess.trim().chars().nth(0).unwrap();
                if g == c{
                    word_container = set_Container(word_container, guess.trim(), x);
                    found += 1;
                    hit_flag = true; 
                }
            }
            if hit_flag == false{
                miss += 1;
            }
            else{
                hit_flag = false; 
            }
        }
        if found >= word_size{
            println!("You have Won Congradulations!");
            break;
        }


    }
}

fn set_Container(container: String, Letter: &str, place: i32) -> String{
    let mut x = 0;
    let mut contain = String::new();
    for C in container.chars(){
        x += 1;
        if x == place{
            contain.push_str(Letter);
        }
        else{
            contain.push(C);
        }

    }
    return contain;

}

fn Hangman_Draw(Miss: i32){
    println!("  ------");
    if Miss >= 1{
        println!("  0    |");
    }
    else{
        println!("       |");
    }

    if Miss >= 2{
        if Miss >= 3{
            if Miss >= 4{
                println!(" -+-   |");
            }
            else {
                println!(" -+    |");
            }
        }
        else {
            println!("  +    |");
        }
    }
    else {
        println!("       |");
    }

    if Miss >= 5{
        println!("  |    |");
    }
    else {
        println!("       |");
    }
    if Miss >= 6{
        if Miss >= 7{
            println!(" ( )   |");
        }
        else{
            println!(" (     |"); 
        }
    }
    else{
        println!("       |");
    }
    println!("       |");
    println!("  -----------");


}
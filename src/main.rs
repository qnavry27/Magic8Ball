 //! Magic 8 Ball.
 //! 
 //! The player has the possiblity to provide any question and in return
 //! receives an randomly chosen answer (yes or no) from 20 possible choices.
 
 use rand::seq::SliceRandom;
 use std::{io, thread, time};

 /// Represents the 20 answers a Magic 8-Ball can give.
 #[derive(Debug, Clone, Copy)]
 enum Answers {
    ItIsCetain,
    ItIsDecidedlySo,
    WithoutADoubt,
    YesDefinitely,
    YouMayRelyOnIt,
    AsIseeItYes,
    MostLikely,
    OutlookGood,
    Yes,
    SignsPointToYes,
    ReplyHazyTryAgain,
    AskAgainLater,
    BetterNotTellYouNow,
    CannotPredcitNow,
    ConcentrateAndAskAgain,
    DontCountOnIt,
    MyReplyIsNo,
    MySourcesSayNo,
    OutlookNotSoGood,
    VeryDoubtful,
 }

 impl Answers {
    /// Returns the human-readable string representation of the 8-Ball answer.
    pub fn as_string(&self) -> &'static str {
        match self {
            Self::ItIsCetain => "It is certain.",
            Self::ItIsDecidedlySo => "It is decidedly so.",
            Self::WithoutADoubt => "Without a doubt.",
            Self::YesDefinitely => "Yes - definitely.",
            Self::YouMayRelyOnIt => "You may rely on it.",
            Self::AsIseeItYes => "As I see it, yes.",
            Self::MostLikely => "Most likely",
            Self::OutlookGood => "Outlook good.",
            Self::Yes => "Yes",
            Self::SignsPointToYes => "Signs point to yes.",
            Self::ReplyHazyTryAgain => "Reply hazy, try again",
            Self::AskAgainLater => "Ask again later",
            Self::BetterNotTellYouNow => "Better not tell you now.",
            Self::CannotPredcitNow => "Cannot predict now.",
            Self::ConcentrateAndAskAgain => "Concentrate and ask again.",
            Self::DontCountOnIt => "Don't count on it.",
            Self::MyReplyIsNo => "My rely is no.",
            Self::MySourcesSayNo => "My sources say no.",
            Self::OutlookNotSoGood => "Outlook not so good.",
            Self::VeryDoubtful => "Very doubtful.",
            
        }
    }

    const ALL: [Answers; 20] = [
        Self::ItIsCetain,
        Self::ItIsDecidedlySo,
        Self::WithoutADoubt,
        Self::YesDefinitely,
        Self::YouMayRelyOnIt,
        Self::AsIseeItYes,
        Self::MostLikely,
        Self::OutlookGood,
        Self::Yes,
        Self::SignsPointToYes,
        Self::ReplyHazyTryAgain,
        Self::AskAgainLater,
        Self::BetterNotTellYouNow,
        Self::CannotPredcitNow,
        Self::ConcentrateAndAskAgain,
        Self::DontCountOnIt,
        Self::MyReplyIsNo,
        Self::MySourcesSayNo,
        Self::OutlookNotSoGood,
        Self::VeryDoubtful,
    ];

    /// Simulates shaking the Magic 8-Ball by randomly selecting one of the 20 answers.
    pub fn shake() -> Self{
        // Randomly choose one of the replies.
        let mut rng = rand::thread_rng();
        *Self::ALL.choose(&mut rng).unwrap()
    }
}

/// Controls the flow of the main game loop.
#[derive(Debug, PartialEq)]
enum GameState{
    Continue,
    Quit,
}
impl GameState{
    /// Prompts the user to decide whether to play another round or quit.
    pub fn get_rematch_status() -> GameState {
        loop{
            println!("Do you want to play another round (y/n):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return GameState::Continue,
                "n" | "no" | "quit" => return GameState::Quit,
                _ => println!("Please type 'y' for yes or 'n' for no.\n"),
            }
        }
    }
}

///Reads a question from the player via standard input.
/// 
/// This function enforces that the input must contain a question mark.
/// If it doesn't, it displays an error and promts the user again.
fn get_player_input() -> String {
    loop {
        println!("Ask a yes/no question");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.contains("?"){
            return input;
        } else {
            println!("Input was not a question, please provide a ?");
            continue
        }
    }   
}
 
 /// The main entry point of the game. 
 /// Manages the primary game loop, time delays, and matching states.
fn main() {

    println!("Welcome to the Magic-8-Ball!\n");
    loop {
        let question = get_player_input();

        println!("Your question is: {question}\n");
        println!("Shaking the Magic 8 Ball...\n");

        thread::sleep(time::Duration::from_millis(1500));

        let answer = Answers::shake();
        println!("Answer: {}", answer.as_string());

        if GameState::get_rematch_status() == GameState::Quit {
            println!("Bye");
            break;
        }
        println!("")
    }
}

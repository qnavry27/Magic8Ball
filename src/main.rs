 /// Magic 8 Ball.
 /// The player has the possiblity to provide any question and in return
 /// receives an randomly chosen answer (yes or no) from 20 possible choices.
 
 use rand::seq::SliceRandom;

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
    pub fn shake() -> Self{
        // Randomly choose one of the replies.
        let mut rng = rand::thread_rng();
        let answers: [Answers; 20] = [
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
        *answers.choose(&mut rng).unwrap()
    }
}
 
fn main() {
    // ADD INPUT
    println!("Your question is");
    println!("Shaking the Magic 8 Ball...\n");

    let answer = Answers::shake();

    println!("Answer: {}", answer.as_string());
}

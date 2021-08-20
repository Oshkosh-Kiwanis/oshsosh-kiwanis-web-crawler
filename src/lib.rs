use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contest {
    pub display_name: String,
    pub page: String,
    pub match_day: usize,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContestGoal {
    pub contest: Contest,
    pub goal: usize,
    pub raised: usize,
    // this is going to be 5,000 because of how the match
    // day will be operated
    pub match_day: usize,
}


// Just an empty struct to serve up all the contest
// that we want to crawl
pub struct Contests {}
impl Contests {
    pub fn get_all() -> Vec<Contest> {
        Vec::from([
            Contest {
                display_name: "NEW Top Dog Neenah".into(),
                page: "newtopdogneenah2022".into(),
                match_day: 0
            },
            Contest {
                display_name: "NEW Top Dog Lakeshore".into(),
                page: "newtopdoglakeshore2022".into(),
                match_day: 0
            }
        ])
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTopDog {
    // the name of the dog
    pub dog: String,
    // how many votes the dog got
    // this also encodes how much the dog has raised
    // $1 = 1 vote
    pub votes: usize,
    // Which contest the dog belongs to
    pub contest: Contest,
    // The page in which someone can go to to see the dog
    pub page: String,
    // The picture url for the dog
    pub picture: String,
}
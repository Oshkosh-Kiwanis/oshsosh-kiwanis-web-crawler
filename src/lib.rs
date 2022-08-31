use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Contest {
    pub display_name: String,
    pub page: String,
    pub champ_day: usize,
    pub num_dogs: usize,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContestData {
    pub contest: Contest,
    pub goal: usize,
    pub raised: usize,
    // Total entries in the contest
    pub total_entries: usize,
    // this will usually just be a hardcoded thing
    pub champ_day: usize,
    // When this data was captured
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ContestDataCSV {
    pub display_name: String,
    pub page: String,
    pub goal: usize,
    pub raised: usize,
    // Total entries in the contest
    pub total_entries: usize,
    // this will usually just be a hardcoded thing
    pub champ_day: usize,
    // When this data was captured
    pub timestamp: i64,
}

impl ContestDataCSV {
    pub fn from_contest_data(data: &ContestData) -> ContestDataCSV {
        ContestDataCSV {
            display_name: data.contest.display_name.clone(),
            page: data.contest.page.clone(),
            goal: data.goal,
            raised: data.raised,
            total_entries: data.total_entries,
            champ_day: data.champ_day,
            timestamp: data.timestamp,
        }
    }
}


// Just an empty struct to serve up all the contest
// that we want to crawl
pub struct Contests {}
impl Contests {
    pub fn get_all() -> Vec<Contest> {
        Vec::from([
            Contest {
                display_name: "Lakeshore Humane Society's NEW Top Dog Fall 2022".into(),
                page: "newtopdoglakeshorefall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            },
            Contest {
                display_name: "Misfit Mutts's NEW Top Dog Fall 2022".into(),
                page: "newtopdogmisfitfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            },
            Contest {
                display_name: "Neenah's NEW Top Dog Fall 2022".into(),
                page: "newtopdogneenahfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            },
            Contest {
                display_name: "Mit Liebe's NEW Top Dog Fall 2022".into(),
                page: "newtopdogmitliebefall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            },
            Contest {
                display_name: "Oshkosh's NEW Top Dog Fall 2022".into(),
                page: "newtopdogoahsfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            },
            Contest {
                display_name: "Sandi Paws's NEW Top Dog Fall 2022".into(),
                page: "newtopdogsandipawsfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            }
        ])
    }

    pub fn from_category(category: &str) -> Option<Contest> {
        let category = category.to_lowercase();

        let mut ret: Option<Contest> = None;

        if category.contains("lakeshore") {
            ret = Some(Contest {
                display_name: "Lakeshore Humane Society's NEW Top Dog Fall 2022".into(),
                page: "newtopdoglakeshorefall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        } else if category.contains("misfit mutt") {
            ret = Some(Contest {
                display_name: "Misfit Mutts's NEW Top Dog Fall 2022".into(),
                page: "newtopdogmisfitfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        } else if category.contains("neenah") {
            ret = Some(Contest {
                display_name: "Neenah's NEW Top Dog Fall 2022".into(),
                page: "newtopdogneenahfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        } else if category.contains("mit liebe") {
            ret = Some(Contest {
                display_name: "Mit Liebe's NEW Top Dog Fall 2022".into(),
                page: "newtopdogmitliebefall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        } else if category.contains("oshskosh") {
            ret = Some(Contest {
                display_name: "Oshkosh's NEW Top Dog Fall 2022".into(),
                page: "newtopdogoahsfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        } else if category.contains("sandi paw") {
            ret = Some(Contest {
                display_name: "Sandi Paws's NEW Top Dog Fall 2022".into(),
                page: "newtopdogsandipawsfall2022".into(),
                champ_day: 0,
                num_dogs: 15,
            });
        }

        ret
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryData {
    // the name of the dog
    pub dog: String,
    // how many votes the dog got
    // this also encodes how much the dog has raised
    pub votes: usize,
    // votes are not 1:1 with money so
    // we need to have a raised value to encode that
    pub raised: usize,
    // Which contest the dog belongs to
    pub contest: Contest,
    // Which category the dog belongs to
    pub category: String,
    // The page in which someone can go to to see the dog
    pub page: String,
    // The picture url for the dog
    pub picture: String,
    // When was this data captured
    pub timestamp: i64,
}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryDataCSV {
    pub display_name: String,
    pub gogophoto_contest_page: String,
    pub dog: String,
    pub votes: usize,
    pub entry_url: String,
    pub picture: String,
    pub timestamp: i64,
}

impl EntryDataCSV {
    pub fn from_entry(entry: &EntryData) -> EntryDataCSV {
        EntryDataCSV {
            display_name: entry.contest.display_name.clone(),
            gogophoto_contest_page: entry.contest.page.clone(),
            dog: entry.dog.clone(),
            votes: entry.votes,
            entry_url: entry.page.clone(),
            picture: entry.picture.clone(),
            timestamp: entry.timestamp,
        }
    }
}

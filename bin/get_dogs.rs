use std::{convert::TryFrom, error::Error};

use chrono::Utc;
use oshkosh_kiwanis_web_crawler::{Contest, Contests, EntryData, EntryDataCSV};
use reqwest::Client;
use tokio::time::{interval, Duration};

use nipper::Document;
use log::{debug, error, info, warn};

async fn crawl_entry_page(client: &Client, domain: &str, webpage: &str, contest: Contest) -> Result<EntryData, Box<dyn Error>> {
    info!("getting url; url={:?}", &webpage);

    let resp  = client.get(webpage).send().await?;
    let html = resp.text().await?;

    let doc = Document::try_from(&html)?;

    let dog: String = doc.select("#form1 > div.main > div.mainBody > div:nth-child(1) > h1")
        .text()
        .split('\n')
        .take(2)
        .collect::<String>()
        .trim()
        .into();

    debug!("selected dog; dog={}", dog);

    let votes: usize = doc.select("h3.viewEntryVotes")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    let raised: usize = doc.select("#ContentPlaceHolder_divRaised > span")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    let category = doc.select("#ContentPlaceHolder_divEntryCategory")
        .text()
        .to_string()
        .replace("Entry Category:", "")
        .trim()
        .to_string();

    debug!("selected votes; votes={}", votes);

    let picture: String = doc.select("#ContentPlaceHolder_imgEntry")
        .attr("src")
        .map_or(String::from(""), |v| v.to_string());

    debug!("selected picture; picture={}", picture);

    let now = Utc::now();
    let timestamp = now.timestamp();

    Ok(EntryData {
        dog,
        votes,
        raised,
        contest,
        category,
        page: String::from(webpage),
        picture: format!("{}{}", domain, picture),
        timestamp,
    })
}

async fn crawl_site(client: &Client, domain: &str, contest: Contest) -> Result<Vec<EntryData>, Box<dyn Error>> {
    // navigate to the search page for the contest,
    // this is where we will grab the top tep results
    let url = format!("{}/{}/search", domain, contest.page);

    info!("getting url; url={:?}", url);

    // get the webapge html
    let resp  = client.get(url).send().await?;
    let html = resp.text().await?;

    // now we have to parse that html
    let doc = Document::try_from(&html)?;

    // go through each of the dogs on the leaderboard of the page
    let mut entry_pages: Vec<String> = vec![];
    let mut dogs = vec![];
    doc.select("#ContentPlaceHolder_upPanel .searchEntryCont a.searchEntry").iter().take(contest.num_dogs).for_each(|entry_link| {
        if let Some(entry_link_str) = entry_link.attr("href") {
            debug!("selected entry; entry_url={}", entry_link_str);
            // navigate to the entry page for easier parsindefaultg
            entry_pages.push(format!("{}{}", domain, entry_link_str));
        }
    });

    for entry_page in entry_pages {
        if let Ok(new_top_dog) = crawl_entry_page(client, domain, &entry_page, contest.clone()).await {
            dogs.push(new_top_dog);
            debug!("successfully crawled entry page; entry_page={}", entry_page);
        } else {
            warn!("something went wrong when trying to crawl the entry page; entry_page={}", entry_page);
        }
    }

    info!("sucessfully got entries; c={}; n={}", contest.display_name, dogs.len());

    Ok(dogs)
}


// lets do some web crawling!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let domain = "https://www.gogophotocontest.com";

    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()?;

    // Do this every minute!
    let mut interval = interval(Duration::from_secs(60));
    'outer: loop {
        interval.tick().await;
        info!("tick");

        let mut results: Vec<EntryData> = Vec::new();
        for contest in Contests::get_all() {
            let ret = match crawl_site(&client, domain, contest).await {
                Ok(res) => res,
                Err(e) => {
                    error!("Unable to crawl site; domain={}; error={}", domain, e);
                    // we encountered an error so lets skip this iteration instead
                    // of just skipping this contest
                    continue 'outer;
                }
            };

            results.extend(ret);
        }

        results.sort_by(|a: &EntryData, b: &EntryData| b.votes.cmp(&a.votes));

        let mut csv_wtr = csv::Writer::from_path("top-dogs.csv")?;
        let mut csv_wtr_global_leaderboard = csv::Writer::from_path("global-leaderboard.csv")?;


        for (i, result) in results.iter().enumerate() {
            let record = EntryDataCSV::from_entry(result);

            // add the top 15 to the global leaderboard
            if i <= 15 {
                csv_wtr.serialize(record.clone())?;
                csv_wtr_global_leaderboard.serialize(record)?;
            } else {
                csv_wtr.serialize(record)?;
            }
        }
        csv_wtr.flush()?;

        debug!("wrote csv file; file=top-dogs.csv");
        debug!("wrote csv file; file=global-leaderboard.csv");

        // write the results to a json file
        let serialized = serde_json::to_string(
            &results
        )?;

        std::fs::write("top-dogs.json", serialized)?;
        debug!("wrote json file; file=top-dogs.json");

        // write the results to the global leaderboard json file
        let serialized_global_leaderboard = serde_json::to_string(
            &results
        )?;

        std::fs::write("global-leaderboard.json", serialized_global_leaderboard)?;
        debug!("wrote json file; file=global-leaderboard.json");

        info!("done");
    }
}

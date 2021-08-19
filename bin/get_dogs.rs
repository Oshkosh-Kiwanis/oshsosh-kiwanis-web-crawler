use std::{convert::TryFrom, error::Error};

use oshkosh_kiwanis_web_crawler::{NewTopDog, Contest, Contests};
use tokio::time::{interval, Duration};

use nipper::Document;

async fn crawl_entry_page(domain: &str, webpage: String, contest: Contest) -> Result<NewTopDog, Box<dyn Error>> {
    println!("[INFO] getting url; url={:?}", &webpage);

    let resp  = reqwest::get(&webpage).await?;
    let html = resp.text().await?;

    let doc = Document::try_from(&html)?;

    let dog: String = doc.select("#form1 > div.main > div.mainBody > div:nth-child(1) > h1")
        .text()
        .split('\n')
        .take(2)
        .collect::<String>()
        .trim()
        .into();

    let votes: usize = doc.select("h3.viewEntryVotes")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    let picture = doc.select("#ContentPlaceHolder_imgEntry")
        .attr("src")
        .unwrap()
        .to_string();

    Ok(NewTopDog {
        dog,
        votes,
        contest: contest,
        page: webpage,
        picture: format!("{}{}", domain, picture),
    })
}

async fn crawl_site(domain: &str, contest: Contest) -> Result<Vec<NewTopDog>, Box<dyn Error>> {
    // navigate to the search page for the contest,
    // this is where we will grab the top tep results
    let url = format!("{}/{}/search", domain, contest.page);

    println!("[INFO] getting url; url={:?}", url);

    // get the webapge html
    let resp  = reqwest::get(url).await?;
    let html = resp.text().await?;

    // now we have to parse that html
    let doc = Document::try_from(&html)?;

    // go through each of the dogs on the leaderboard of the page
    let mut entry_pages: Vec<String> = vec![];
    let mut dogs = vec![];
    doc.select("#ContentPlaceHolder_upPanel .searchEntryCont a.searchEntry").iter().take(10).for_each(|entry_link| {
        if let Some(entry_link_str) = entry_link.attr("href") {
            // navigate to the entry page for easier parsindefaultg
            entry_pages.push(format!("{}{}", domain, entry_link_str));

        }
    });

    for entry_page in entry_pages {
        if let Ok(new_top_dog) = crawl_entry_page(domain, entry_page, contest.clone()).await {
            dogs.push(new_top_dog);
        }
    }

    println!("[INFO] sucessfully got entries; c={}; n={}", contest.display_name, dogs.len());

    Ok(dogs)
}


// lets do some web crawling!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let domain = "https://www.gogophotocontest.com";

    // Do this every minute!
    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        println!("[TICK]");

        let mut results: Vec<NewTopDog> = Vec::new();
        for contest in Contests::get_all() {
            let ret = crawl_site(domain, contest).await?;

            results.extend(ret);
        }

        // write the results to a json file
        let serialized = serde_json::to_string(
            &results
        )?;

        std::fs::write("top-dogs.json", serialized)?;
        println!("[DONE]");
    }
}

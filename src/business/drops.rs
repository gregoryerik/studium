//
//  downloading and storing the words and their translations taken from drops
//  the words are grouped in categories
//
//

// not a finished url. e.g. https://languagedrops.com/word/en/english/german/
const URL: &str = "https://languagedrops.com/word/en/english/";



pub async fn scrape_drops_language(lang: &str) {
    let full_url = format!("{URL}{lang}/");

    println!("Full URL: {}", full_url);

    scrape::get_topics(full_url).await;
}

mod scrape {
    use std::collections::HashMap;

    // A collection of topics under a single heading
    // e.g. food and drinks as the group with Veg, fruits, cooking etc as the topics
    #[derive(Debug)]
    pub struct SuperGroup {
        name: String,
        groups: Vec<Group>,
    }
    impl SuperGroup {
        pub fn new() -> Self {
            SuperGroup { name: String::new(), groups: Vec::new() }
        }

        pub fn describe(&self) {
            println!("{}", self.name);
            for group in &self.groups {
                println!("- {}", group.name);
                for topic in &group.topics {
                    println!("        -{}", topic.name);
                }
            }
        }
    }


    #[derive(Debug)]
    pub struct Group {
        name: String,
        topics: Vec<Topic>,
    }

    impl Group {
        pub fn new() -> Self {
            Group { name: String::new(),topics: Vec::new() }
        }
    }

    #[derive(Debug)]
    pub struct Topic {
        name: String,
        url: String
    }

    async fn get_page(url: String) -> scraper::Html {
        let response = reqwest::get(url).await.unwrap().text().await.unwrap();

        let document = scraper::Html::parse_document(&response);

        document
    }

    pub async fn get_topics(url: String) -> SuperGroup {
        let mut language_group = SuperGroup::new();
        language_group.name = url.clone();

        let document = get_page(url).await;

        let group_container_selector = scraper::Selector::parse("div.category-container").unwrap();
        let title_selector = scraper::Selector::parse("a.category-title-link").unwrap();
        let item_selector = scraper::Selector::parse("a.linkable-word-box-container").unwrap();

        for group in document.select(&group_container_selector) {
            let mut item_group = Group::new();
            let fragment = scraper::Html::parse_fragment(&group.html());
            let title = fragment
                .select(&title_selector)
                .next()
                .unwrap()
                .inner_html();

            item_group.name = title;
            
            let item_select = fragment.select(&item_selector);

            item_select.for_each(|it| {
                let t = Topic {
                    name: it.value().attr("title").unwrap().to_string(),
                    url: it.value().attr("href").unwrap().to_string()
                };

                item_group.topics.push(t);
 
             });

             language_group.groups.push(item_group);

        };

        //let group_categories_selector = scraper::Selector::parse("a.category-title-link").unwrap();
        //let group_catergoies = document.select(&group_categories_selector).map(|x| x.inner_html());

        //group_catergoies.for_each(|title| println!("{title}"));



        language_group.describe();

        language_group
    }
}

use std::io::Write;

fn main() {
    let lager = vec![
    "33 Export",
    "Desperados",
    "Goldberg",
    "Gulder",
    "Heineken",
    "Star",
    ];

    let stout = vec![
    "Legend",
    "Turbo King",
    "Williams",
    ];

    let Non-Alcoholic = vec![
    "Maltina",
    "Amstel Malta",
    "Malta Gold",
    "Fayrouz",
    ];

    let mut file = std::fs::File::create("nigerian_breweries.txt")
    .expect("create failed");

    let mut file_content = String::new();

    file_content.push_str("lager           |Stout              | Non-Alcoholic\n");
    file_content.push_push_str("------------------------------------\n");

    Let max_rows = Lager.len().max(non_alcoholic.len());

    for i in 0..max_rows {
        let l_item = Lager.get(i).unwrap_or(&"");
        let s_item = stout.get(i).unwrap_or(&"");
        let na_item = non_alcoholic.get(i).unwrap_or(&"");

    Let row = format!("{:<20} | {:<20}  | {:<20}\n", l_item, s_item, na_item);
    file_content.push_str(&row);
    }

    file.write_all(file_content.as_bytes())
    .expect("write failed");

    println!("\nData written to file successfully!");
}

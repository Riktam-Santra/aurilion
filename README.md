# Aurilion - A Rust library for easier interaction with Data Dragon

Aurilion is a small rust library that provides some useful functions to hep facilitate easier access to the Riot Games Data Dragon Web API.

## Usage

Get started by adding the library to your project.
```
cargo add aurilion
```

## Getting the data

At the time of writing, there are only four functions included:

- `get_versions()` - To get all patch versions.
- `get_all_champions()` - To get brief information about all available champions.
- `get_single_champion()` - To get detailed information about a single champion.
- `get_all_items()` - To get detailed information about all items.

## Example

An example of implementation of all the functions would be something like this:

```rs
async fn do_everything() {
    // get all versions
    let versions = get_versions().await.unwrap();

    //The first version in the list is always the latest version
    let latest_version = versions.first().unwrap();

    // get all champions, this function requires the version you are requesting champions for
    let all_champions = get_all_champions(latest_version.clone()).await.unwrap();

    /*
        do something here with the champions

        we cycle through and list out all champion names and their title for now.
    */

    all_champions.data.into_iter().for_each(|value| {
        println!("{} : {}", value.1.id, value.1.title);
    });

    let single_champion = get_single_champion("Ahri".to_string(), latest_version.clone())
        .await
        .unwrap();

    // Get the champion from the Map received (must be the same as supplied to get_single_champion())
    // In our case, it is "Ahri"
    let champion_data = single_champion.data.get("Ahri").unwrap();
    // Just print the name of the champions and their skills for now.
    println!("\n\n{} Skill Names: ", champion_data.id);
    for (num, x) in champion_data.spells.clone().into_iter().enumerate() {
        println!("Skill {}: {}", num + 1, x.name)
    }
}
```
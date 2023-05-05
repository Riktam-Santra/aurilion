use super::*;

#[tokio::test]
async fn getting_versions() {
    let result = get_versions().await.unwrap();
    for x in &result {
        println!("{}", x);
    }
    assert_eq!(result.into_iter().len(), 418);
}

#[tokio::test]
async fn getting_all_champions() {
    let result = get_all_champions(String::from("13.9.1")).await.unwrap();
    assert_ne!(result.data.get("Ahri"), None);
}

#[tokio::test]
async fn getting_one_champion() {
    let result = get_single_champion(String::from("Yasuo"), String::from("13.9.1"))
        .await
        .unwrap();
    println!("{:#?}", result);
}

#[tokio::test]
async fn getting_all_items() {
    let result = get_all_items(String::from("13.9.1")).await.unwrap();
    println!("{:#?}", result);
}

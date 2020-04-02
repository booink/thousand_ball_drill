// 01. 「パタトクカシーー」
//
// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
//

fn extract_from_string() {
    let mut even = false;
    let extracted_string = "パタトクカシーー".chars().filter(|_| {
        even = !even;
        even
    }).collect::<String>();
    assert_eq!(extracted_string, "パトカー");
}

fn extract_from_string2() {
    let extracted_string = "パタトクカシーー".char_indices().filter(|(i, _)| i % 2 == 0).map(|(_, c)| c).collect::<String>();
    assert_eq!(extracted_string, "パトカー");
}

pub fn answer() {
    extract_from_string();
    extract_from_string2();
}

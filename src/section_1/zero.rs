// 第1章: 準備運動
// 00. 文字列の逆順
//
// 文字列"stressed"の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
//

fn reverse_string() {
    let reversed_string = "stressed".chars().rev().collect::<String>();
    assert_eq!(reversed_string, "desserts");
}

pub fn answer() {
    reverse_string();
}

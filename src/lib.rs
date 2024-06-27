mod scanner;
mod parser;
mod evaluator;

use std::{env, fs::File, io::prelude::*};

use scanner::scanner::Scanner;
// use parser::parser::parse;
// use evaluator::evaluator::Evaluator;

pub fn execute() {
    let args = env::args().collect::<Vec<String>>();
    let filename = &args[1];

    let mut f = File::open(filename).expect("[Error] ファイルが見つかりませんでした。");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("[Error] ファイル読み込み時に何らかのエラーが発生しました。");

    let mut sc = Scanner::new(&contents);
    // let ast = parse(&mut sc);
    // let result = Evaluator::apply(&ast);
    // println!("計算結果: {}", result);
}

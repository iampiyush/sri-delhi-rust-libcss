//////////////////////////////////////////////////////////////////////
//
// Filename         : csdetect.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
FIXME
* Do not understand: different function prototype for
   css__parser_create("UTF-8", CSS_CHARSET_DICTATED, myrealloc, NULL, &parser)
   pub fn css_parser_create(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc>);

*/



extern mod std;
extern mod css;
extern mod parserutils;


//use std::arc;
//use css::parse::*;
//use parserutils::charset::csdetect::*;

fn main() {
    io::println("csdetect");
}

#[test]
fn csdetect() {
    //css__parse_filesize();
}
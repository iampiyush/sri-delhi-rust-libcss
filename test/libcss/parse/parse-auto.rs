extern mod std;
extern mod css;
extern mod wapcaplet;
// extern mod test;

// use test::*;
use core::io::*;
use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
// use css::include::types::*;
use wapcaplet::*;


pub fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) -> (css_result,Option<arc::RWARC<~lwc_string>>) {
	return (CSS_OK,Some(rel.clone()));
}

pub fn fill_params() -> css_params {
	let css_param = css_params {
		params_version : 1,
		level: CSS_LEVEL_21,
		charset : Some(~"UTF-8"),
		url : ~"foo",
		title : ~"",
		allow_quirks : false,
		inline_style : false,
		resolve : @resolve_url,
		import : None,
		color : None,
		font : None
	};
	return css_param;
}

fn main() {
    io::println("parse");
}

fn css_create_fn() -> ~css{
	let css = css_create(fill_params());
	css
}

fn parse(file_name: ~str) {

	let mut css = css_create_fn();
	let CHUNK_SIZE = 10;
	let mut buf: ~[u8];
	let r:@Reader = io::file_reader(&Path(file_name)).get(); 
	r.seek(0 , SeekEnd);
	let mut len = r.tell();
	r.seek(0 , SeekSet);

	while len>0 {
		buf = r.read_bytes(CHUNK_SIZE as uint);
		len -= buf.len();
		let error = css.css_stylesheet_append_data(buf);
		match error {
			CSS_OK => {},
			CSS_NEEDDATA => {},
			_ => {assert!(false);}
		}
	}
	buf = r.read_bytes(len as uint);
	let error = css.css_stylesheet_append_data(buf);
	match error {
		CSS_OK => {},
		CSS_NEEDDATA => {},
		_ => {assert!(false);}
	}
	let css_stylesheet = css.css_stylesheet_data_done();

	match css_stylesheet.url {
		~"foo" => {},
		_ => {assert!(false);}
	}
	match css_stylesheet.title {
		~"" => {},
		_ => {assert!(false);}
	}
}

//  validate rules here



#[test]
fn parse_tests1() {
	parse(~"../data/parse/tests1.dat");
}

#[test]
fn parse_atrules() {
	parse(~"../data/parse/atrules.dat");
}

#[test]
fn parse_colours() {
	parse(~"../data/parse/colours.dat");
}

#[test]
fn parse_colours_hsl() {
	parse(~"../data/parse/colours-hsl.dat");
}

#[test]
fn parse_nth() {
	parse(~"../data/parse/nth.dat");
}

#[test]
fn parse_properties() {
	parse(~"../data/parse/properties.dat");
}

#[test]
fn parse_selectors() {
	parse(~"../data/parse/selectors.dat");
}
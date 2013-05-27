extern mod std;
extern mod wapcaplet;
extern mod parserutils;

use std::arc;
use wapcaplet::*;
use parserutils::input::inputstream::*;
use parserutils::utils::error::*;

fn css_error_from_parserutils_error(css_result : parserutils_error) -> css_error{
	match css_result{
		CSS_OK => { 
			return CSS_OK
		},
		UTILS_NOMEM => {
			return CSS_NOMEM
		},
		PARSERUTILS_BADPARM => {
			return CSS_BADPARM
		},
		PARSERUTILS_INVALID => {
			return CSS_INVALID
		},
		PARSERUTILS_FILENOTFOUND => {
			return CSS_FILENOTFOUND
		},
		PARSERUTILS_NEEDDATA => {
			return CSS_NEEDDATA
		},
		PARSERUTILS_BADCHARSET => {
			return CSS_BADCHARSET
		},
		PARSERUTILS_EOF => {
			return CSS_EOF
		},
	}
}
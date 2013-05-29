extern mod std;
extern mod css; // To be removed
extern mod wapcaplet;
extern mod parserutils;

use std::arc;
use wapcaplet::*;
use parserutils::input::inputstream::*;
use parserutils::charset::encodings::utf8impl::*;
use parserutils::utils::errors::*;

use css::utils::errors::*;
use css::utils::parserutilserror::*;


pub enum css_token_type {
    CSS_TOKEN_IDENT,
    CSS_TOKEN_ATKEYWORD,
    CSS_TOKEN_HASH,
    CSS_TOKEN_FUNCTION,
    CSS_TOKEN_STRING,
    CSS_TOKEN_INVALID_STRING,
    CSS_TOKEN_URI,
    CSS_TOKEN_UNICODE_RANGE,
    CSS_TOKEN_CHAR,
    CSS_TOKEN_NUMBER,
    CSS_TOKEN_PERCENTAGE,
    CSS_TOKEN_DIMENSION,

    /* Those tokens that want strings interned appear above */
    
    CSS_TOKEN_CDO,
    CSS_TOKEN_CDC,
    CSS_TOKEN_S,
    CSS_TOKEN_COMMENT,
    CSS_TOKEN_INCLUDES,
    CSS_TOKEN_DASHMATCH,
    CSS_TOKEN_PREFIXMATCH, 
    CSS_TOKEN_SUFFIXMATCH,
    CSS_TOKEN_SUBSTRINGMATCH,
    CSS_TOKEN_EOF 
}

pub static CSS_TOKEN_LAST_INTERN:css_token_type = CSS_TOKEN_CDO;

pub struct css_token_data {
    data: ~[u8],
    len: uint
}

pub struct css_token {
    
    data: css_token_data,

    token_type: css_token_type,
    idata: Option<arc::RWARC<~lwc_string>>,

    col: uint,
    line: uint
}

pub enum states {
    sSTART      =  0,
    sATKEYWORD  =  1,
    sSTRING     =  2,
    sHASH       =  3,
    sNUMBER     =  4, 
    sCDO        =  5,
    sCDC        =  6,
    sS      =  7,
    sCOMMENT    =  8,
    sMATCH      =  9, 
    sURI        = 10,
    sIDENT      = 11,
    sESCAPEDIDENT   = 12,
    sURL        = 13,
    sUCR        = 14 
}

struct _context {
    first: u8,      // first character read from token
    orig_bytes: uint,       // storage of cuurent number of bytes read for rewinding
    last_was_star: bool,
    last_was_cr: bool,
    bytes_for_url: uint,
    data_len_for_url: uint,
    hex_count: int
}

static _state: uint = 4;
static _sub_state: uint = 4;

pub struct css_lexer {
    input: ~inputstream,
    bytes_read_for_token: uint,
    token: Option<~css_token>,
    escape_seen: bool,
    unescaped_token_data: Option<~[u8]>,  // used if eascapeSeen  = true
    state: states,
    substate: uint,
    context: _context,
    emit_comments: bool,
    current_col: uint,
    current_line: uint
}

// pub fn preprocess(input: &str) -> ~str {
//  str::replace(str::replace(str::replace(input,
//  "\r\n", "\n"),
//  "\r", "\n"),
//  "\x00", "\uFFFD")
// }

impl css_lexer {
    pub fn css__lexer_create(inputstream: ~inputstream) -> ~css_lexer {
        let _data = css_token_data {
            data: ~[],
            len: 0
        };
        let _token = ~css_token {
            data: _data,
            token_type: CSS_TOKEN_EOF,
            idata: None,
            col: 0,
            line: 0
        };
        let context_inst = _context {
            first: 0,
            orig_bytes: 0,
            last_was_star: false,
            last_was_cr: false,
            bytes_for_url: 0,
            data_len_for_url: 0,
            hex_count: 0
        };
        ~css_lexer{ 
            input: inputstream,
            bytes_read_for_token: 0,
            token: Some(_token),
            escape_seen: false,
            unescaped_token_data: Some(~[]),
            state: sSTART,
            substate: 0,
            emit_comments: false,
            context: context_inst,      
            current_col: 1,
            current_line: 1,
        }
    }

    pub fn css__lexer_get_token(&mut self) -> (css_error , Option<css_token>){

        match self.state {
            sSTART => {},
            sATKEYWORD => {},
            sSTRING => {},
            sHASH => {},
            sNUMBER => {},
            sCDO => {},
            sCDC => {},
            sS => {},
            sCOMMENT => {},
            sMATCH => {},
            sURI => {},
            sIDENT => {},
            sESCAPEDIDENT => {},
            sURL => {},
            sUCR => {}
        }
        return (CSS_INVALID , None);
    }

    /******************************************************************************
     * Utility routines                                                           *
     ******************************************************************************/

    pub fn APPEND(&mut self, data: &[u8], len: uint) {
        self.append_to_token_data(data, len);

        self.bytes_read_for_token += len;
        self.current_col += len;
    }

    pub fn append_to_token_data(&mut self , data: &[u8], len: uint) {
        
        if self.escape_seen {
            self.unescaped_token_data.get_mut_ref().push_all(data.slice(0,len));
        }

        self.token.get_mut_ref().data.len += len;
    }

    pub fn emit_token(&mut self , token_type: css_token_type) -> (css_error, Option<~css_token>) {

        let mut t = self.token.swap_unwrap();
        t.token_type = token_type;

        if (self.escape_seen) {
            t.data.data = self.unescaped_token_data.swap_unwrap();
        }
        else {
            let (pu_peek_result, pu_peek_error) = self.input.parserutils_inputstream_peek(0);

            assert!((token_type as int == CSS_TOKEN_EOF as int) || 
                (pu_peek_error as int == PARSERUTILS_OK as int));

            match token_type {
                CSS_TOKEN_EOF => {
                    t.data.data = ~[];
                }
                _ => {
                    let (cptr, _) = pu_peek_result.unwrap();
                    t.data.data = cptr.slice(0, t.data.len).to_owned();
                }
            }
        }

        match token_type {
            CSS_TOKEN_ATKEYWORD => {
                /* Strip the '@' from the front */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_STRING => {
                /* Strip the leading quote */
                vec::shift(&mut t.data.data);
                t.data.len -=1;

                /* Strip the trailing quote, iff it exists (may have hit EOF) */
                if (t.data.len > 0) {
                    let last = t.data.data.pop();

                    if (last == '"' as u8 || last =='\'' as u8) {
                        t.data.len -=1;
                    }
                    else {
                        t.data.data.push(last);
                    }
                }
            },
            CSS_TOKEN_INVALID_STRING => {
                /* Strip the leading quote */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_HASH => {
                /* Strip the '#' from the front */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_PERCENTAGE => {
                /* Strip the '%' from the end */
                t.data.data.pop();
                t.data.len -=1;
            },
            CSS_TOKEN_DIMENSION => {},
            CSS_TOKEN_URI => {
                /* Strip the "url(" from the start */
                t.data.data = vec::tailn(t.data.data, 4).to_owned();
                t.data.len -= 4;

                /* Strip any leading whitespace */
                /* Strip any leading quote */
                /* Strip the trailing ')' */
                /* Strip any trailing whitespace */
                /* Strip any trailing quote */
                do vec::retain(&mut t.data.data) |&c| {
                    if (c != ' ' as u8 && c != ')' as u8 && c !='"' as u8 && c != '\'' as u8) {
                        true
                    }
                    else {
                        false
                    }
                }

                t.data.len = t.data.data.len();

            },
            CSS_TOKEN_UNICODE_RANGE => {
                /* Remove "U+" from the start */
                t.data.data = vec::tailn(t.data.data, 2).to_owned();
                t.data.len -= 2;
            },
            CSS_TOKEN_COMMENT => {
                /* Strip the leading '/' and '*' */
                /* Strip the trailing '*' and '/' */

                t.data.data = t.data.data.slice(2, t.data.data.len()-2).to_owned();
                t.data.len-=4;
            },
            CSS_TOKEN_FUNCTION => {
                /* Strip the trailing '(' */
                t.data.data.pop();
                t.data.len -= 1;
            },
            _=> {

            }
        }
        self.state = sSTART;
        self.substate = 0;

        return (CSS_OK,Some(t));
    }

    /******************************************************************************
     * State machine components                                                   *
     ******************************************************************************/

    /******************************************************************************
     * Input consumers                                                            *
     ******************************************************************************/ 

    pub fn consume_digits(&mut self) -> css_error {
        /* digit = [0-9] */

        /* Consume all digits */
        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (char::is_digit(c)) {
                self.APPEND(cptr, clen);
            }
            else {
                break;
            }
        }

        return CSS_OK;
    }

    fn consume_escape(&mut self, nl : bool) -> css_error {

        /* escape = unicode | '\' [^\n\r\f0-9a-fA-F] 
             * 
             * The '\' has been consumed.
             */
        
        let (pu_peek_result , perror) = 
            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
        
        match perror {
        
            PARSERUTILS_NOMEM | 
            PARSERUTILS_BADPARM | 
            PARSERUTILS_INVALID | 
            PARSERUTILS_FILENOTFOUND | 
            PARSERUTILS_NEEDDATA | 
            PARSERUTILS_BADENCODING => {
                return css_error_from_parserutils_error(perror);
            }
            PARSERUTILS_EOF => {
               return CSS_EOF;
            }
            _ => {
                /* continue */
            }
        }

        let (cptr , clen) = pu_peek_result.unwrap();
        let mut c = cptr[0] as char;

        if (!nl && (c=='\n' || c=='\r' /* || c=='\f'*/)) {
            /* These are not permitted */
            return CSS_INVALID;
        }

        /* Create unescaped buffer, if it doesn't already exist */
        if (self.unescaped_token_data.is_none()) {
            self.unescaped_token_data = Some(~[]);
        }

        /* If this is the first escaped character we've seen for this token,
         * we must copy the characters we've read to the unescaped buffer */
        if (!self.escape_seen) {
            if (self.bytes_read_for_token > 1) {
                let (pu_peek_result , perror) = 
                    self.input.parserutils_inputstream_peek(0);

                assert!(perror as int == PARSERUTILS_OK as int);

                let (sdata , _) = pu_peek_result.unwrap();
                /* -1 to skip '\\' */
                self.unescaped_token_data.get_mut_ref().push_all(sdata.slice(0, self.bytes_read_for_token-1));
            }

            self.token.get_mut_ref().data.len = self.bytes_read_for_token-1;
            self.escape_seen = true;
        }

        if (char::is_digit_radix(c,16)) {
            self.bytes_read_for_token += clen;

            match (self.consume_unicode(char::to_digit(c, 16).unwrap() as u32)) {
                CSS_OK => {
                    /* continue */
                }
                x => {
                    self.bytes_read_for_token -= clen;
                    return x;
                }
            }
        }

        /* If we're handling escaped newlines, convert CR(LF)? to LF */
        if (nl && c=='\r') {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token+clen);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                c='\n';
                self.APPEND(&[c as u8], 1);

                self.current_col = 1;
                self.current_line += 1;

                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();

            c=cptr[0] as char;

            if (c=='\n') {
                self.APPEND(cptr, 1);
                /* And skip the '\r' in the input */
                self.bytes_read_for_token += clen;

                self.current_col = 1;
                self.current_line += 1;

                return CSS_OK;
            }
            
            self.APPEND(cptr, clen); // cptr has been redefined above
            return CSS_OK;
        }
        else if (nl && (c == '\n'/* || c == '\f'*/)) {
            /* APPEND will increment this appropriately */
            self.current_col = 0;
            self.current_line+=1;
        }
        else if (c != '\n' && c != '\r' /*&& c != '\f'*/) {
            self.current_col+=1;
        }

        /* Append the unescaped character */
        self.APPEND(cptr, clen);

        CSS_OK
    }

    pub fn consume_NM_chars(&mut self) -> css_error
    {
        /* nmchar = [a-zA-Z] | '-' | '_' | nonascii | escape */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (start_nm_char(c) && c != '\\') {
                self.APPEND(cptr, clen);
            }

            if (c == '\\') {
                self.bytes_read_for_token += clen;

                let error = self.consume_escape(false);
                if (error as int != CSS_OK as int) {
                    /* Rewind '\\', so we do the 
                     * right thing next time */
                    self.bytes_read_for_token -= clen;

                    /* Convert either EOF or INVALID into OK.
                     * This will cause the caller to believe that
                     * all NMChars in the sequence have been 
                     * processed (and thus proceed to the next
                     * state). Eventually, the '\\' will be output
                     * as a CHAR. */
                    if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                        return CSS_OK;
                    }

                    return error;
                }
            }

            if (!start_nm_char(c)) {
                break;
            }
        }

        return CSS_OK;
    }

    pub fn consume_string(&mut self) -> css_error
    {
        
        let quote = self.context.first as char;
        let permittedquote = 
            match(quote) {
                '"' => '\'',
                _ => '"'
            };

        /* string = '"' (stringchar | "'")* '"' | "'" (stringchar | '"')* "'"
         *
         * The open quote has been consumed.
         */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == permittedquote) {
                self.APPEND(cptr, clen);
            } else if (start_string_char(c)) {
                let error = self.consume_string_chars();
                if (error as int!= CSS_OK as int) {
                    return error;
                }
            } else if (c != quote) {
                /* Invalid character in string */
                return CSS_INVALID;
            }

            if (c == quote) {
                /* Append closing quote to token data */
                self.APPEND(cptr, clen);
                break;
            }
        }

        return CSS_OK;
    }

    pub fn consume_string_chars(&mut self) -> css_error
    {
        /* stringchar = urlchar | ' ' | ')' | '\' nl */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (start_string_char(c) && c != '\\') {
                self.APPEND(cptr, clen);
            }

            if (c == '\\') {
                self.bytes_read_for_token += clen;

                let error = self.consume_escape(true);
                if (error as int != CSS_OK as int) {
                    /* Rewind '\\', so we do the 
                     * right thing next time. */
                    self.bytes_read_for_token -= clen;

                    /* Convert EOF to OK. This causes the caller
                     * to believe that all StringChars have been
                     * processed. Eventually, the '\\' will be
                     * output as a CHAR. */
                    if (error as int == CSS_EOF as int) {
                        return CSS_OK;
                    }

                    return error;
                }
            }

            if (!start_string_char(c)) {
                break;
            }
        }

        return CSS_OK;
    }

    fn consume_unicode(&mut self, mut ucs : u32) -> css_error {
        let cptr : @mut u8;
        let mut count : int = 0;
        let mut bytes_read_init : uint = self.bytes_read_for_token;

        while (count < 5) {
           let (pu_peek_result , error) = 
            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
        
                PARSERUTILS_NOMEM | 
                PARSERUTILS_BADPARM | 
                PARSERUTILS_INVALID | 
                PARSERUTILS_FILENOTFOUND | 
                PARSERUTILS_NEEDDATA | 
                PARSERUTILS_BADENCODING => {
                    self.bytes_read_for_token = bytes_read_init;
                    return css_error_from_parserutils_error(error);
                }
                PARSERUTILS_EOF => {
                   break;
                }
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    if char::is_digit_radix(_cptr[0] as char, 16){
                        self.bytes_read_for_token += clen;
                        ucs = (ucs << 4) | u32::from_str_radix(str::from_char(_cptr[0] as char), 16).unwrap();
                    }
                    else{
                        break;
                    }
                }

            }

            count += 1;

        }

        if (ucs > 0x10FFFF || ucs <= 0x0008 || ucs == 0x000B ||
                (0x000E <= ucs && ucs <= 0x001F) ||
                (0x007F <= ucs && ucs <= 0x009F) ||
                (0xD800 <= ucs && ucs <= 0xDFFF) ||
                (0xFDD0 <= ucs && ucs <= 0xFDEF) ||
                (ucs & 0xFFFE) == 0xFFFE) {
            ucs = 0xFFFD;
        } else if (ucs == 0x000D) {
            ucs = 0x000A;
        }

        let (utf8sequence_option, pu_charset_error) = parserutils_charset_utf8_from_ucs4(ucs);
        match (pu_charset_error) {
            PARSERUTILS_OK => {
                let (pu_peek_result , error) = 
                 self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
                 match error {
                
                     PARSERUTILS_NOMEM | 
                     PARSERUTILS_BADPARM | 
                     PARSERUTILS_INVALID | 
                     PARSERUTILS_FILENOTFOUND | 
                     PARSERUTILS_NEEDDATA | 
                     PARSERUTILS_BADENCODING => {
                         self.bytes_read_for_token = bytes_read_init;
                         return css_error_from_parserutils_error(error);
                     }
                     PARSERUTILS_EOF => {
                        return CSS_OK;
                     }
                     PARSERUTILS_OK => {
                         let mut (_cptr , clen) = pu_peek_result.unwrap();
                         if (_cptr[0] as char == '\r') { // Potential CRLF 
                             //let mut p_cr : u8 = _cptr[0];
                             let (pu_peek_result2 , error2) = 
                               self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
                             self.bytes_read_for_token = bytes_read_init;

                             match error2 {
                            
                                 PARSERUTILS_NOMEM | 
                                 PARSERUTILS_BADPARM | 
                                 PARSERUTILS_INVALID | 
                                 PARSERUTILS_FILENOTFOUND | 
                                 PARSERUTILS_NEEDDATA | 
                                 PARSERUTILS_BADENCODING => {
                                     self.bytes_read_for_token = bytes_read_init;
                                     return css_error_from_parserutils_error(error2);
                                 }
                                 PARSERUTILS_EOF => {
                                    return CSS_OK;
                                 }
                                 PARSERUTILS_OK => {
                                     let (_cptr2 , clen2) = pu_peek_result2.unwrap();
                                     if (_cptr2[0] as char == '\n') { // Potential CRLF 
                                         self.bytes_read_for_token += 1;
                                         _cptr = _cptr2;
                                     }
                                 }
                             }
                         }
                         let mut utf8sequence = utf8sequence_option.unwrap();
                         self.append_to_token_data(utf8sequence, utf8sequence.len());
                         if (is_space(_cptr[0] as char)) {
                            self.bytes_read_for_token += clen;
                         }

                         if _cptr[0] as char=='\r' || _cptr[0] as char == '\n' /*|| _cptr[0] == '\f'*/ {
                            self.current_col = 1;
                            self.current_line += 1;
                         }
                         else {
                            self.current_col += self.bytes_read_for_token - bytes_read_init + 2;
                         }
                     }

                 }
            }
            _ => {
                return css_error_from_parserutils_error(pu_charset_error);
            }
        }

        CSS_OK
    }

    pub fn consume_url_chars(&mut self) -> css_error {
        loop {
            let (pu_peek_result , error) = self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    let c = _cptr[0] as char;
                    
                    if start_url_char(c) && c != '\\' {
                        self.append_to_token_data(_cptr, clen);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen;
                    }

                    if c == ('\\') {
                        self.bytes_read_for_token += clen;
                        let lex_error = self.consume_escape(false);

                        match lex_error {
                            CSS_OK => {},
                            _ => {
                                self.bytes_read_for_token -= clen;
                                return CSS_OK;
                            }
                        }
                    }

                    if !start_url_char(c) {
                        break;
                    }
                }

                _ => {
                    return CSS_INVALID;
                },

            }

            
        }

        return CSS_OK
    } 

    pub fn consume_w_chars(&mut self) -> css_error {
        loop {
            let (pu_peek_result , error) = self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    let c = _cptr[0] as char;
                    
                    if is_space(c) {
                        self.append_to_token_data(_cptr, clen);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen;
                    }

                    if c == ('\n'){
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    if (self.context.last_was_cr && c != ('\n')) {
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    self.context.last_was_cr = (c == '\r');

                    if !is_space(c) {
                        break;
                    }
                },

                _ => {
                    return CSS_INVALID;
                }

            }

        }

        if self.context.last_was_cr {
            self.current_col = 1;
            self.current_line += 1;
        }

        CSS_OK
    }

} // impl css_lexer




fn start_nm_char(c: char) -> bool{
    return c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || 
        ('0' <= c && c <= '9') || c == '-' || c >= 0x80 as char || c == '\\';
}

fn start_nm_start(c: char) -> bool{
    return c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') ||
        c >= 0x80 as char || c == '\\';
}

fn start_string_char(c: char) -> bool{
    return start_url_char(c) || c == ' ' || c == ')';
}

fn start_url_char(c: char) -> bool{
    return c == '\t' || c == '!' || ('#' <= c && c <= '&') || c == '(' ||
        ('*' <= c && c <= '~') || c >= 0x80 as char || c == '\\';
}

fn is_space(c: char) -> bool{
    return c == ' ' || c == '\r' || c == '\n' || c == '\t';
}


fn main() {
    io::println("lexer");
}
use wapcaplet::*;
use std::arc;

use parserutils::input::inputstream::*;

// libcss uses
use charset::csdetect::*;
use lex::lexer::*;
use parse::language::*;
use parse::parse::*;
use stylesheet::*;
use utils::errors::*;

pub struct css {
	lwc:arc::RWARC<~lwc>,
	stylesheet:@mut css_stylesheet,
	parser:~css_parser,
	css_create_lwc_time: float,
	css_create_inputstream_time: float,
	css_create_lexer_time: float,
	css_create_stylesheet_time:float,
	css_create_language_time:float,
	css_create_parser_time:float,
	css_create_inputstream_alias_time:float,
}

enum css_params_version {
	CSS_PARAMS_VERSION_1 = 1
}

pub struct css_params {
		/** ABI version of this structure */
		params_version : css_params_version,

		/** The language level of the stylesheet */
		level: css_language_level,

		/** The charset of the stylesheet data, or NULL to detect */
		charset : Option<~str>,
		/** URL of stylesheet */
		url : @str,
		/** Title of stylesheet */
		title : @str,

		/** Permit quirky parsing of stylesheet */
		allow_quirks : bool,
		/** This stylesheet is an inline style */
		inline_style : bool,

		/** URL resolution function */
		resolve : css_url_resolution_fn,

		/** Import notification function */
		import : Option<css_import_notification_fn>,

		/** Colour resolution function */
		color : Option<css_color_resolution_fn>,

		/** Font resolution function */
		font : Option<css_font_resolution_fn>,
}

pub impl css {
	pub fn css_create(params: &css_params, lwc_instance: Option<arc::RWARC<~lwc>>) -> @mut css {
            	let mut start_time = std::time::precise_time_ns();
		// create lwc
		let lwc = 	if lwc_instance.is_none() { 
						lwc()
					}  
					else {
						lwc_instance.get_ref().clone()
					} ;
            	let mut end_time = std::time::precise_time_ns();
	        let create_lwc_time = (end_time as float - start_time as float);


		// create inputstream
            	let mut start_time = std::time::precise_time_ns();
		let (inputstream_option, _) =  
			match copy params.charset {
				None => inputstream(None, None ,Some(~css__charset_extract)),
				Some(charset) => inputstream(Some(charset), Some(CSS_CHARSET_DICTATED as int), Some(~css__charset_extract))
			};
		

            	let mut end_time = std::time::precise_time_ns();
	        let create_input_stream_time = (end_time as float - start_time as float);

		// create lexer
		
            	let mut start_time = std::time::precise_time_ns();
		let lexer = css_lexer::css__lexer_create(inputstream_option.unwrap());
            	let mut end_time = std::time::precise_time_ns();
	        let create_lexer_time = (end_time as float - start_time as float);
		let input_stream_alias_time = lexer.input.inputstream_alias_create_time;

		// create stylesheet
            	let mut start_time = std::time::precise_time_ns();
		let stylesheet = @mut css_stylesheet {
			selectors:css_selector_hash::css__selector_hash_create(),       
			rule_count:0,                        
			rule_list:None,   
			last_rule:None,   
			disabled:false,                          
			url:params.url,                               
			title:params.title,                             
			level:params.level,               
			quirks_allowed:params.allow_quirks,                    
			quirks_used:false,                       
			inline_style:params.inline_style,                      
			cached_style:None,    
			string_vector:~[],
			resolve : params.resolve, 
			import : params.import, 
			font : params.font,   
			color: params.color
		};
            	let mut end_time = std::time::precise_time_ns();
	        let create_stylesheet_time = (end_time as float - start_time as float);

		// create language
            	let mut start_time = std::time::precise_time_ns();
		let language = css_language(stylesheet, lwc.clone());
            	let mut end_time = std::time::precise_time_ns();
	        let create_language_time = (end_time as float - start_time as float);

		// create parser
            	let mut start_time = std::time::precise_time_ns();
		let parser = match params.inline_style {
		    false => css_parser::css__parser_create(language, lexer, lwc.clone()),
		    true => css_parser::css__parser_create_for_inline_style(language, lexer, lwc.clone())
		}; 
            	let mut end_time = std::time::precise_time_ns();
	        let create_parser_time = (end_time as float - start_time as float);

		@mut css {
			lwc:lwc.clone(),
			parser:parser.unwrap(),
			stylesheet:stylesheet,
        	        css_create_lwc_time:create_lwc_time,
        		css_create_inputstream_time:create_input_stream_time,
        		css_create_lexer_time:create_lexer_time,
        		css_create_stylesheet_time:create_stylesheet_time,
        		css_create_language_time:create_language_time,
        		css_create_parser_time:create_parser_time,
			css_create_inputstream_alias_time:input_stream_alias_time,
		}
	}

    /**
    * #Description:
    *   Append source data to a stylesheet.
	
    * #Arguments:
    *  'data' - The data to append.
    
	* #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
	pub fn css_stylesheet_append_data(&mut self, data:~[u8]) -> css_error {
		self.parser.css__parser_parse_chunk(data)
	}

    /**
    * #Description:
    *   Flag that the last of a stylesheet's data has been seen.
	
	* #Return Value:
    *   'css_error' - CSS_OK on success,
					  CSS_IMPORTS_PENDING if there are imports pending,
					  appropriate error otherwise.
    */
	pub fn css_stylesheet_data_done(&mut self) -> css_error {
		let error = self.parser.css__parser_completed();
		match error {
			CSS_OK=>{},
			err => {
				return err ;
			}
		}

		self.stylesheet.cached_style = None;

		let mut ptr = self.stylesheet.rule_list ;
		loop {
			match ptr {
				None=>{
					return CSS_OK ;
				},
				Some(rule)=>{
					match rule {
						RULE_IMPORT(import_rule)=>{
							if import_rule.sheet.is_none() {
								return CSS_IMPORTS_PENDING ;
							}
							else {
								ptr = css_stylesheet::css__stylesheet_get_base_rule(rule).next;
								loop ;
							}
						},
						RULE_UNKNOWN(_)=>{
							ptr = css_stylesheet::css__stylesheet_get_base_rule(rule).next;
							loop ;
						},
						RULE_CHARSET(_)=>{
							ptr = css_stylesheet::css__stylesheet_get_base_rule(rule).next;
							loop ;
						},
						_=>{
							break ;
						}
					}
				}
			}
		}

		CSS_OK
	}

    /**
    * #Description:
    *   Set a stylesheet's disabled state.
	
    * #Arguments:
    *  'disabled' - The new disabled state.
    
	* #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
	pub fn css_stylesheet_set_disabled(&mut self, disabled:bool ) -> css_error {

	    self.stylesheet.disabled = disabled;
	    CSS_OK
	}

    /**
    * #Description:
    *   Get disabled status of a stylesheet.
	
	* #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , disabled state flag).
    */
	pub fn css_stylesheet_get_disabled(&mut self) -> (css_error,bool) {

	    (CSS_OK,self.stylesheet.disabled)
	}

    /**
    * #Description:
    *   Determine whether quirky parsing was permitted on a stylesheet.
	
	* #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , quirks allowed flag).
    */
	pub fn css_stylesheet_quirks_allowed(&mut self) -> (css_error,bool) {

	    (CSS_OK,self.stylesheet.quirks_allowed)
	}

    /**
    * #Description:
    *   Determine whether quirky parsing was used on a stylesheet.
	
	* #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , quirks used flag).
    */
	pub fn css_stylesheet_used_quirks(&mut self) -> (css_error,bool) {

	    (CSS_OK,self.stylesheet.quirks_used)
	}

    /**
    * #Description:
    *   Retrieve the title associated with a stylesheet.
	
	* #Return Value:
    *   '(css_error,~str)' - (CSS_OK , title).
    */
	pub fn css_stylesheet_get_title(&mut self) -> (css_error,@str) {

	    (CSS_OK,self.stylesheet.title)
	}

    /**
    * #Description:
    *   Retrieve the URL associated with a stylesheet.
	
	* #Return Value:
    *   '(css_error,~str)' - (CSS_OK , url).
    */
	pub fn css_stylesheet_get_url(&mut self) -> (css_error,@str) {

	    (CSS_OK,self.stylesheet.url)
	}

    /**
    * #Description:
    *   Retrieve the language level of a stylesheet.
	
	* #Return Value:
    *   '(css_error,css_language_level)' - (CSS_OK , level).
    */
	pub fn css_stylesheet_get_language_level(&mut self) -> 
	                                (css_error,css_language_level) {

	    (CSS_OK,self.stylesheet.level)  
	}

    /**
    * #Description:
    *   Retrieve the next pending import for the parent stylesheet.
	* The client must resolve the absolute URL of the imported stylesheet,
	* using the parent's URL as the base. It must then fetch the imported
	* stylesheet, and parse it to completion, including fetching any stylesheets
	* it may import. The resultant sheet must then be registered with the
	* parent using css_stylesheet_register_import().
	*
	* The client must then call this function again, to determine if there
	* are any further imports for the parent stylesheet, and, if so,
	* process them as described above.
	*
	* If the client is unable to fetch an imported stylesheet, it must
	* register an empty stylesheet with the parent in its place.
    
	* #Return Value:
    *   '(css_error,Option<~str>,Option<u64>)' - (CSS_OK, URL of imported stylesheet, applicable media types for 
														imported stylesheet) on success, 
												(appropriate error, None, None) otherwise.
    */
	pub fn css_stylesheet_next_pending_import(&mut self) -> 
	                            (css_error,Option<@str>,Option<u64>) {

	    let mut ptr = self.stylesheet.rule_list ;
	    loop {
	        match ptr {
	            None=> {
	                break ;
	            },
	            Some(current_rule) => {
	                match current_rule {
	                    RULE_IMPORT(irule)=>{
	                        if irule.sheet.is_none() {
	                            return (CSS_OK,Some(irule.url),Some(irule.media));
	                        }
	                        else {
	                            ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                            loop ;
	                        }
	                    },
	                    RULE_CHARSET(_) =>{
	                        ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                        loop;
	                    },
	                    RULE_UNKNOWN(_) =>{
	                        ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                        loop;
	                    },
	                    _=> {
	                        break ;
	                    }
	                }
	            }
	        }
	    }
	    (CSS_INVALID,None,None) 
	}

    /**
    * #Description:
    *   Register an imported stylesheet with its parent.
	
    * #Arguments:
    *  'import' - Imported sheet.
    
	* #Return Value:
    *   'css_error' - CSS_OK on success, CSS_INVALID if there are no outstanding imports, appropriate error otherwise.
    */
	pub fn css_stylesheet_register_import(&mut self, import:Option<@mut css_stylesheet>) 
	    -> css_error {
	    	debug!("Entering: css_stylesheet_register_import");

	    if import.is_none() {
	    	// debug!("Entering: import.is_none()");
	        return CSS_BADPARM ;
	    }

	    let mut ptr = self.stylesheet.rule_list ;
	    loop {
	        match ptr {
	            None=> {
	                break ;
	            },
	            Some(current_rule) => {
	                match current_rule {

	                    RULE_IMPORT(irule)=>{
	                        if irule.sheet.is_none() {
	                            irule.sheet = import ;
	                            return CSS_OK ;
	                        }
	                        else {
	                            ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                            loop ;
	                        }
	                    },
	                    RULE_CHARSET(_) =>{
	                        ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                        loop;
	                    },
	                    RULE_UNKNOWN(_) =>{
	                        ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
	                        loop;
	                    },
	                    _=> {
	                        break ;
	                    }
	                }
	            }
	        }
	    }
	    CSS_INVALID 
	}

}

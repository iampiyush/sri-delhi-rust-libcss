use std::arc;
use wapcaplet::*;
use stylesheet::*;

pub enum index_property {
     // Universal selector 
    UNIVERSAL ,
     
     // At-rules 
    CHARSET,
    LIBCSS_IMPORT, 
    MEDIA, 
    NAMESPACE, 
    FONT_FACE, 
    PAGE,

     // Media types 
    AURAL, 
    BRAILLE, 
    EMBOSSED, 
    HANDHELD, 
    PRINT, 
    PROJECTION, 
    SCREEN, 
    SPEECH, 
    TTY, 
    TV, 
    ALL,

     // Pseudo classes 
    FIRST_CHILD, 
    LINK, 
    VISITED, 
    HOVER, 
    ACTIVE, 
    FOCUS, 
    LANG, 

     // LEFT, RIGHT, -- already in properties  
    FIRST,
    ROOT, 
    NTH_CHILD, 
    NTH_LAST_CHILD, 
    NTH_OF_TYPE, 
    NTH_LAST_OF_TYPE,
    LAST_CHILD, 
    FIRST_OF_TYPE, 
    LAST_OF_TYPE, 
    ONLY_CHILD,
    ONLY_OF_TYPE, 
    EMPTY, 
    TARGET, 
    ENABLED, 
    DISABLED, 
    CHECKED, 
    NOT, 

     // Pseudo elements 
    FIRST_LINE, 
    FIRST_LETTER, 
    BEFORE, 
    AFTER,

     // Properties 
    // FIRST_PROP = AZIMUTH,

    AZIMUTH, 
    BACKGROUND, 
    BACKGROUND_ATTACHMENT, 
    BACKGROUND_COLOR, 
    BACKGROUND_IMAGE, 
    BACKGROUND_POSITION, 
    BACKGROUND_REPEAT, 
    BORDER, 
    BORDER_BOTTOM, 
    BORDER_BOTTOM_COLOR, 
    BORDER_BOTTOM_STYLE, 
    BORDER_BOTTOM_WIDTH, 
    BORDER_COLLAPSE, 
    BORDER_COLOR, 
    BORDER_LEFT, 
    BORDER_LEFT_COLOR, 
    BORDER_LEFT_STYLE, 
    BORDER_LEFT_WIDTH, 
    BORDER_RIGHT, 
    BORDER_RIGHT_COLOR, 
    BORDER_RIGHT_STYLE, 
    BORDER_RIGHT_WIDTH, 
    BORDER_SPACING, 
    BORDER_STYLE, 
    BORDER_TOP, 
    BORDER_TOP_COLOR, 
    BORDER_TOP_STYLE, 
    BORDER_TOP_WIDTH, 
    BORDER_WIDTH, 
    BOTTOM, 
    BREAK_AFTER,
    BREAK_BEFORE,
    BREAK_INSIDE, 
    CAPTION_SIDE, 
    CLEAR, 
    CLIP, 
    COLOR, 
    COLUMNS, 
    COLUMN_COUNT,
    COLUMN_FILL, 
    COLUMN_GAP, 
    COLUMN_RULE, 
    COLUMN_RULE_COLOR,
    COLUMN_RULE_STYLE, 
    COLUMN_RULE_WIDTH, 
    COLUMN_SPAN, 
    COLUMN_WIDTH,
    CONTENT, 
    COUNTER_INCREMENT, 
    COUNTER_RESET, 
    CUE, 
    CUE_AFTER, 
    CUE_BEFORE,
    CURSOR, 
    DIRECTION, 
    DISPLAY, 
    ELEVATION, 
    EMPTY_CELLS, 
    LIBCSS_FLOAT,
    FONT,
    FONT_FAMILY, 
    FONT_SIZE, 
    FONT_STYLE, 
    FONT_VARIANT, 
    FONT_WEIGHT, 
    HEIGHT,
    LEFT, 
    LETTER_SPACING, 
    LINE_HEIGHT, 
    LIST_STYLE, 
    LIST_STYLE_IMAGE,
    LIST_STYLE_POSITION, 
    LIST_STYLE_TYPE, 
    MARGIN, 
    MARGIN_BOTTOM,
    MARGIN_LEFT, 
    MARGIN_RIGHT, 
    MARGIN_TOP, 
    MAX_HEIGHT, 
    MAX_WIDTH,
    MIN_HEIGHT, 
    MIN_WIDTH, 
    OPACITY, 
    ORPHANS, 
    OUTLINE, 
    OUTLINE_COLOR,
    OUTLINE_STYLE, 
    OUTLINE_WIDTH, 
    OVERFLOW, 
    PADDING, 
    PADDING_BOTTOM,
    PADDING_LEFT, 
    PADDING_RIGHT, 
    PADDING_TOP, 
    PAGE_BREAK_AFTER,
    PAGE_BREAK_BEFORE, 
    PAGE_BREAK_INSIDE, 
    PAUSE, 
    PAUSE_AFTER, 
    PAUSE_BEFORE,
    PITCH_RANGE, 
    PITCH, 
    PLAY_DURING, 
    POSITION, 
    QUOTES, 
    RICHNESS, 
    RIGHT,
    SPEAK_HEADER, 
    SPEAK_NUMERAL, 
    SPEAK_PUNCTUATION, 
    SPEAK, 
    SPEECH_RATE,
    STRESS, 
    TABLE_LAYOUT, 
    TEXT_ALIGN, 
    TEXT_DECORATION, 
    TEXT_INDENT,
    TEXT_TRANSFORM, 
    TOP, 
    UNICODE_BIDI, 
    VERTICAL_ALIGN, 
    VISIBILITY,
    VOICE_FAMILY, 
    VOLUME, 
    WHITE_SPACE, 
    WIDOWS, 
    WIDTH, 
    WORD_SPACING, 
    Z_INDEX,

    // LAST_PROP = Z_INDEX as int,

     // Other keywords 
    INHERIT, 
    IMPORTANT, 
    NONE, 
    BOTH, 
    FIXED, 
    SCROLL, 
    TRANSPARENT,
    NO_REPEAT, 
    REPEAT_X, 
    REPEAT_Y, 
    REPEAT, 
    HIDDEN,
    DOTTED, 
    DASHED,
    SOLID, 
    LIBCSS_DOUBLE, 
    GROOVE, 
    RIDGE, 
    INSET, 
    OUTSET, 
    THIN, 
    MEDIUM, 
    THICK,
    COLLAPSE, 
    SEPARATE, 
    AUTO, 
    LTR, 
    RTL, 
    INLINE, 
    BLOCK, 
    LIST_ITEM, 
    RUN_IN,
    INLINE_BLOCK, 
    TABLE, 
    INLINE_TABLE, 
    TABLE_ROW_GROUP, 
    TABLE_HEADER_GROUP,
    TABLE_FOOTER_GROUP, 
    TABLE_ROW, 
    TABLE_COLUMN_GROUP, 
    TABLE_COLUMN,
    TABLE_CELL, 
    TABLE_CAPTION,
    BELOW, 
    LEVEL, 
    ABOVE, 
    HIGHER, 
    LOWER,
    SHOW, 
    HIDE, 
    XX_SMALL, 
    X_SMALL, 
    SMALL, 
    LARGE, 
    X_LARGE,
    XX_LARGE,
    LARGER, 
    SMALLER, 
    NORMAL, 
    ITALIC, 
    OBLIQUE, 
    SMALL_CAPS, 
    BOLD, 
    BOLDER,
    LIGHTER, 
    INSIDE,
    OUTSIDE, 
    DISC, 
    CIRCLE, 
    SQUARE, 
    DECIMAL, 
    DECIMAL_LEADING_ZERO, 
    LOWER_ROMAN, 
    UPPER_ROMAN, 
    LOWER_GREEK,
    LOWER_LATIN,
    UPPER_LATIN, 
    ARMENIAN, 
    GEORGIAN, 
    LOWER_ALPHA, 
    UPPER_ALPHA,
    INVERT, 
    VISIBLE, 
    ALWAYS, 
    AVOID, 
    X_LOW, 
    LOW, 
    HIGH, 
    X_HIGH, 
    LIBCSS_STATIC,
    RELATIVE, 
    ABSOLUTE, 
    ONCE, 
    DIGITS, 
    CONTINUOUS, 
    CODE, 
    SPELL_OUT, 
    X_SLOW,
    SLOW, 
    FAST, 
    X_FAST, 
    FASTER, 
    SLOWER, 
    CENTER, 
    JUSTIFY, 
    CAPITALIZE,
    UPPERCASE, 
    LOWERCASE, 
    EMBED, 
    BIDI_OVERRIDE, 
    BASELINE, 
    SUB, 
    SUPER,
    TEXT_TOP, 
    MIDDLE, 
    TEXT_BOTTOM, 
    SILENT, 
    X_SOFT, 
    SOFT, 
    LOUD, 
    X_LOUD,
    PRE, 
    NOWRAP, 
    PRE_WRAP, 
    PRE_LINE, 
    LEFTWARDS, 
    RIGHTWARDS, 
    LEFT_SIDE,
    FAR_LEFT, 
    CENTER_LEFT, 
    CENTER_RIGHT, 
    FAR_RIGHT, 
    RIGHT_SIDE, 
    BEHIND, 
    RECT, 
    OPEN_QUOTE, 
    CLOSE_QUOTE, 
    NO_OPEN_QUOTE, 
    NO_CLOSE_QUOTE, 
    ATTR, 
    COUNTER, 
    COUNTERS, 
    CROSSHAIR, 
    DEFAULT, 
    POINTER, 
    MOVE, 
    E_RESIZE, 
    NE_RESIZE, 
    NW_RESIZE, 
    N_RESIZE, 
    SE_RESIZE, 
    SW_RESIZE, 
    S_RESIZE, 
    W_RESIZE, 
    LIBCSS_TEXT, 
    WAIT, 
    HELP, 
    PROGRESS, 
    SERIF, 
    SANS_SERIF, 
    CURSIVE,
    FANTASY, 
    MONOSPACE, 
    MALE, 
    FEMALE, 
    CHILD, 
    MIX, 
    UNDERLINE, 
    OVERLINE, 
    LINE_THROUGH, 
    BLINK, 
    RGB, 
    RGBA,
    HSL, 
    HSLA, 
    LIBCSS_LEFT, 
    LIBCSS_CENTER,
    LIBCSS_RIGHT, 
    CURRENTCOLOR, 
    ODD, 
    EVEN, 
    SRC, 
    LOCAL, 
    INITIAL,
    FORMAT, 
    WOFF, 
    TRUETYPE, 
    OPENTYPE, 
    EMBEDDED_OPENTYPE, 
    SVG, 
    COLUMN,
    AVOID_PAGE, 
    AVOID_COLUMN,
    BALANCE,

     // Named colours 
    //FIRST_COLOUR = ALICEBLUE,

    ALICEBLUE, 
    ANTIQUEWHITE, 
    AQUA, 
    AQUAMARINE, 
    AZURE,
    BEIGE, 
    BISQUE, 
    BLACK, 
    BLANCHEDALMOND, 
    BLUE, 
    BLUEVIOLET, 
    BROWN,
    BURLYWOOD, 
    CADETBLUE, 
    CHARTREUSE, 
    CHOCOLATE, 
    CORAL, 
    CORNFLOWERBLUE,
    CORNSILK, 
    CRIMSON, 
    CYAN, 
    DARKBLUE, 
    DARKCYAN, 
    DARKGOLDENROD, 
    DARKGRAY,
    DARKGREEN, 
    DARKGREY, 
    DARKKHAKI, 
    DARKMAGENTA, 
    DARKOLIVEGREEN, 
    DARKORANGE,
    DARKORCHID, 
    DARKRED, 
    DARKSALMON, 
    DARKSEAGREEN, 
    DARKSLATEBLUE,
    DARKSLATEGRAY, 
    DARKSLATEGREY, 
    DARKTURQUOISE, 
    DARKVIOLET, 
    DEEPPINK,
    DEEPSKYBLUE, 
    DIMGRAY, 
    DIMGREY, 
    DODGERBLUE, 
    FELDSPAR, 
    FIREBRICK,
    FLORALWHITE, 
    FORESTGREEN, 
    FUCHSIA, 
    GAINSBORO, 
    GHOSTWHITE, 
    GOLD, 
    GOLDENROD, 
    GRAY, 
    GREEN, 
    GREENYELLOW, 
    GREY, 
    HONEYDEW, 
    HOTPINK,
    INDIANRED, 
    INDIGO, 
    IVORY, 
    KHAKI, 
    LAVENDER, 
    LAVENDERBLUSH, 
    LAWNGREEN,
    LEMONCHIFFON, 
    LIGHTBLUE, 
    LIGHTCORAL, 
    LIGHTCYAN, 
    LIGHTGOLDENRODYELLOW,
    LIGHTGRAY, 
    LIGHTGREEN, 
    LIGHTGREY, 
    LIGHTPINK, 
    LIGHTSALMON, 
    LIGHTSEAGREEN,
    LIGHTSKYBLUE, 
    LIGHTSLATEBLUE, 
    LIGHTSLATEGRAY, 
    LIGHTSLATEGREY, 
    LIGHTSTEELBLUE, 
    LIGHTYELLOW, 
    LIME, 
    LIMEGREEN, 
    LINEN, 
    MAGENTA, 
    MAROON,
    MEDIUMAQUAMARINE, 
    MEDIUMBLUE, 
    MEDIUMORCHID, 
    MEDIUMPURPLE, 
    MEDIUMSEAGREEN, 
    MEDIUMSLATEBLUE, 
    MEDIUMSPRINGGREEN, 
    MEDIUMTURQUOISE,
    MEDIUMVIOLETRED, 
    MIDNIGHTBLUE, 
    MINTCREAM, 
    MISTYROSE, 
    MOCCASIN,
    NAVAJOWHITE, 
    NAVY, 
    OLDLACE, 
    OLIVE, 
    OLIVEDRAB, 
    ORANGE, 
    ORANGERED,
    ORCHID, 
    PALEGOLDENROD, 
    PALEGREEN, 
    PALETURQUOISE, 
    PALEVIOLETRED,
    PAPAYAWHIP, 
    PEACHPUFF, 
    PERU, 
    PINK, 
    PLUM, 
    POWDERBLUE, 
    PURPLE, 
    RED,
    ROSYBROWN, 
    ROYALBLUE, 
    SADDLEBROWN, 
    SALMON, 
    SANDYBROWN, 
    SEAGREEN,
    SEASHELL, 
    SIENNA, 
    SILVER, 
    SKYBLUE, 
    SLATEBLUE, 
    SLATEGRAY, 
    SLATEGREY,
    SNOW, 
    SPRINGGREEN, 
    STEELBLUE, 
    TAN, 
    TEAL,
    THISTLE, 
    TOMATO, 
    TURQUOISE, 
    VIOLET, 
    VIOLETRED, 
    WHEAT, 
    WHITE, 
    WHITESMOKE, 
    YELLOW, 
    YELLOWGREEN,

    // LAST_COLOUR = YELLOWGREEN as int,

    LAST_KNOWN
}


pub struct css_propstrings {
    lwc_instance: arc::RWARC<~lwc>,
    propstrings: ~[arc::RWARC<~lwc_string>],
    pseudo_class_list:~[index_property],
    pseudo_element_list:~[index_property]
}

impl css_propstrings {

    pub fn css_propstrings(lwc_instance: arc::RWARC<~lwc>) -> ~css_propstrings {
        let mut css_propstrings_instance = ~css_propstrings {
            lwc_instance: lwc_instance.clone(),
            propstrings: ~[],
            pseudo_class_list : ~[ 
                                    FIRST_CHILD,
                                    LINK,
                                    VISITED,
                                    HOVER,
                                    ACTIVE,
                                    FOCUS,
                                    LANG,
                                    LEFT,
                                    RIGHT,
                                    FIRST,
                                    ROOT,
                                    NTH_CHILD,
                                    NTH_LAST_CHILD,
                                    NTH_OF_TYPE,
                                    NTH_LAST_OF_TYPE,
                                    LAST_CHILD,
                                    FIRST_OF_TYPE,
                                    LAST_OF_TYPE,
                                    ONLY_CHILD,
                                    ONLY_OF_TYPE,
                                    EMPTY,
                                    TARGET,
                                    ENABLED,
                                    DISABLED,
                                    CHECKED,
                                    NOT,
                                ],

            pseudo_element_list : ~[
                                    FIRST_LINE, 
                                    FIRST_LETTER, 
                                    BEFORE, 
                                    AFTER,
                                ]
        };

        let mut propstrings_list = ~[~"yellowgreen", ~"yellow", ~"whitesmoke", ~"white", ~"wheat", ~"violetred", ~"violet", ~"turquoise", ~"tomato", ~"thistle", ~"teal", ~"tan", ~"steelblue", ~"springgreen", ~"snow", ~"slategrey", ~"slategray", ~"slateblue", ~"skyblue", ~"silver", ~"sienna", ~"seashell", ~"seagreen", ~"sandybrown", ~"salmon", ~"saddlebrown", ~"royalblue", ~"rosybrown", ~"red", ~"purple", ~"powderblue", ~"plum", ~"pink", ~"peru", ~"peachpuff", ~"papayawhip", ~"palevioletred", ~"paleturquoise", ~"palegreen", ~"palegoldenrod", ~"orchid", ~"orangered", ~"orange", ~"olivedrab", ~"olive", ~"oldlace", ~"navy", ~"navajowhite", ~"moccasin", ~"mistyrose", ~"mintcream", ~"midnightblue", ~"mediumvioletred", ~"mediumturquoise", ~"mediumspringgreen", ~"mediumslateblue", ~"mediumseagreen", ~"mediumpurple", ~"mediumorchid", ~"mediumblue", ~"mediumaquamarine", ~"maroon", ~"magenta", ~"linen", ~"limegreen", ~"lime", ~"lightyellow", ~"lightsteelblue", ~"lightslategrey", ~"lightslategray", ~"lightslateblue", ~"lightskyblue", ~"lightseagreen", ~"lightsalmon", ~"lightpink", ~"lightgrey", ~"lightgreen", ~"lightgray", ~"lightgoldenrodyellow", ~"lightcyan", ~"lightcoral", ~"lightblue", ~"lemonchiffon", ~"lawngreen", ~"lavenderblush", ~"lavender", ~"khaki", ~"ivory", ~"indigo", ~"indianred", ~"hotpink", ~"honeydew", ~"grey", ~"greenyellow", ~"green", ~"gray", ~"goldenrod", ~"gold", ~"ghostwhite", ~"gainsboro", ~"fuchsia", ~"forestgreen", ~"floralwhite", ~"firebrick", ~"feldspar", ~"dodgerblue", ~"dimgrey", ~"dimgray", ~"deepskyblue", ~"deeppink", ~"darkviolet", ~"darkturquoise", ~"darkslategrey", ~"darkslategray", ~"darkslateblue", ~"darkseagreen", ~"darksalmon", ~"darkred", ~"darkorchid", ~"darkorange", ~"darkolivegreen", ~"darkmagenta", ~"darkkhaki", ~"darkgrey", ~"darkgreen", ~"darkgray", ~"darkgoldenrod", ~"darkcyan", ~"darkblue", ~"cyan", ~"crimson", ~"cornsilk", ~"cornflowerblue", ~"coral", ~"chocolate", ~"chartreuse", ~"cadetblue", ~"burlywood", ~"brown", ~"blueviolet", ~"blue", ~"blanchedalmond", ~"black", ~"bisque", ~"beige", ~"azure", ~"aquamarine", ~"aqua", ~"antiquewhite", ~"aliceblue", ~"balance", ~"avoid-column", ~"avoid-page", ~"column", ~"svg", ~"embedded-opentype", ~"opentype", ~"truetype", ~"woff", ~"format", ~"initial", ~"local", ~"src", ~"even", ~"odd", ~"currentColor", ~"-libcss-right", ~"-libcss-center", ~"-libcss-left", ~"hsla", ~"hsl", ~"rgba", ~"rgb", ~"blink", ~"line-through", ~"overline", ~"underline", ~"mix", ~"child", ~"female", ~"male", ~"monospace", ~"fantasy", ~"cursive", ~"sans-serif", ~"serif", ~"progress", ~"help", ~"wait", ~"text", ~"w-resize", ~"s-resize", ~"sw-resize", ~"se-resize", ~"n-resize", ~"nw-resize", ~"ne-resize", ~"e-resize", ~"move", ~"pointer", ~"default", ~"crosshair", ~"counters", ~"counter", ~"attr", ~"no-close-quote", ~"no-open-quote", ~"close-quote", ~"open-quote", ~"rect", ~"behind", ~"right-side", ~"far-right", ~"center-right", ~"center-left", ~"far-left", ~"left-side", ~"rightwards", ~"leftwards", ~"pre-line", ~"pre-wrap", ~"nowrap", ~"pre", ~"x-loud", ~"loud", ~"soft", ~"x-soft", ~"silent", ~"text-bottom", ~"middle", ~"text-top", ~"super", ~"sub", ~"baseline", ~"bidi-override", ~"embed", ~"lowercase", ~"uppercase", ~"capitalize", ~"justify", ~"center", ~"slower", ~"faster", ~"x-fast", ~"fast", ~"slow", ~"x-slow", ~"spell-out", ~"code", ~"continuous", ~"digits", ~"once", ~"absolute", ~"relative", ~"static", ~"x-high", ~"high", ~"low", ~"x-low", ~"avoid", ~"always", ~"visible", ~"invert", ~"upper-alpha", ~"lower-alpha", ~"georgian", ~"armenian", ~"upper-latin", ~"lower-latin", ~"lower-greek", ~"upper-roman", ~"lower-roman", ~"decimal-leading-zero", ~"decimal", ~"square", ~"circle", ~"disc", ~"outside", ~"inside", ~"lighter", ~"bolder", ~"bold", ~"small-caps", ~"oblique", ~"italic", ~"normal", ~"smaller", ~"larger", ~"xx-large", ~"x-large", ~"large", ~"small", ~"x-small", ~"xx-small", ~"hide", ~"show", ~"lower", ~"higher", ~"above", ~"level", ~"below", ~"table-caption", ~"table-cell", ~"table-column", ~"table-column-group", ~"table-row", ~"table-footer-group", ~"table-header-group", ~"table-row-group", ~"inline-table", ~"table", ~"inline-block", ~"run-in", ~"list-item", ~"block", ~"inline", ~"rtl", ~"ltr", ~"auto", ~"separate", ~"collapse", ~"thick", ~"medium", ~"thin", ~"outset", ~"inset", ~"ridge", ~"groove", ~"double", ~"solid", ~"dashed", ~"dotted", ~"hidden", ~"repeat", ~"repeat-y", ~"repeat-x", ~"no-repeat", ~"transparent", ~"scroll", ~"fixed", ~"both", ~"none", ~"important", ~"inherit", ~"z-index", ~"word-spacing", ~"width", ~"widows", ~"white-space", ~"volume", ~"voice-family", ~"visibility", ~"vertical-align", ~"unicode-bidi", ~"top", ~"text-transform", ~"text-indent", ~"text-decoration", ~"text-align", ~"table-layout", ~"stress", ~"speech-rate", ~"speak", ~"speak-punctuation", ~"speak-numeral", ~"speak-header", ~"right", ~"richness", ~"quotes", ~"position", ~"play-during", ~"pitch", ~"pitch-range", ~"pause-before", ~"pause-after", ~"pause", ~"page-break-inside", ~"page-break-before", ~"page-break-after", ~"padding-top", ~"padding-right", ~"padding-left", ~"padding-bottom", ~"padding", ~"overflow", ~"outline-width", ~"outline-style", ~"outline-color", ~"outline", ~"orphans", ~"opacity", ~"min-width", ~"min-height", ~"max-width", ~"max-height", ~"margin-top", ~"margin-right", ~"margin-left", ~"margin-bottom", ~"margin", ~"list-style-type", ~"list-style-position", ~"list-style-image", ~"list-style", ~"line-height", ~"letter-spacing", ~"left", ~"height", ~"font-weight", ~"font-variant", ~"font-style", ~"font-size", ~"font-family", ~"font", ~"float", ~"empty-cells", ~"elevation", ~"display", ~"direction", ~"cursor", ~"cue-before", ~"cue-after", ~"cue", ~"counter-reset", ~"counter-increment", ~"content", ~"column-width", ~"column-span", ~"column-rule-width", ~"column-rule-style", ~"column-rule-color", ~"column-rule", ~"column-gap", ~"column-fill", ~"column-count", ~"columns", ~"color", ~"clip", ~"clear", ~"caption-side", ~"break-inside", ~"break-before", ~"break-after", ~"bottom", ~"border-width", ~"border-top-width", ~"border-top-style", ~"border-top-color", ~"border-top", ~"border-style", ~"border-spacing", ~"border-right-width", ~"border-right-style", ~"border-right-color", ~"border-right", ~"border-left-width", ~"border-left-style", ~"border-left-color", ~"border-left", ~"border-color", ~"border-collapse", ~"border-bottom-width", ~"border-bottom-style", ~"border-bottom-color", ~"border-bottom", ~"border", ~"background-repeat", ~"background-position", ~"background-image", ~"background-color", ~"background-attachment", ~"background", ~"azimuth", ~"after", ~"before", ~"first-letter", ~"first-line", ~"not", ~"checked", ~"disabled", ~"enabled", ~"target", ~"empty", ~"only-of-type", ~"only-child", ~"last-of-type", ~"first-of-type", ~"last-child", ~"nth-last-of-type", ~"nth-of-type", ~"nth-last-child", ~"nth-child", ~"root", ~"first", ~"lang", ~"focus", ~"active", ~"hover", ~"visited", ~"link", ~"first-child", ~"all", ~"tv", ~"tty", ~"speech", ~"screen", ~"projection", ~"print", ~"handheld", ~"embossed", ~"braille", ~"aural", ~"page", ~"font-face", ~"namespace", ~"media", ~"import", ~"charset", ~"*"];

        let mut i =0;
        let length = propstrings_list.len();
        while(i < length) {
            css_propstrings_instance.propstrings.push(
                do (lwc_instance).write |s| {
                    s.lwc_intern_string(propstrings_list.pop())
                }
            );
            i += 1;
        }
        css_propstrings_instance
    }


    pub fn lwc_string_caseless_isequal(&mut self , lwc_string_instance: arc::RWARC<~lwc_string> , string_index: uint) -> bool {

        let lwc_instance = self.lwc_instance.clone();
        do lwc_instance.write |l| {
            l.lwc_string_caseless_isequal(lwc_string_instance.clone() , self.propstrings[string_index].clone())
        }
    }

    pub fn lwc_string_isequal(&mut self , lwc_string_instance: arc::RWARC<~lwc_string> , string_index: uint) -> bool {
        let lwc_instance = self.lwc_instance.clone();
        do lwc_instance.read |l| {
            l.lwc_string_isequal(lwc_string_instance.clone() , self.propstrings[string_index].clone())
        }
    }

    pub fn lwc_string_data(&mut self, string_index:uint) -> ~str {
        lwc_string_data(self.propstrings[string_index].clone())
    }

    pub fn is_selector_pseudo(&mut self, name: ~str) -> Option<(css_selector_type, index_property)> {
        
        let mut return_value : Option<(css_selector_type, index_property)> = None;

        do (self.lwc_instance).write |l| {
            let name_intern = l.lwc_intern_string(copy name);

            for self.pseudo_class_list.each |&string_index| {
                if  (
                        l.lwc_string_caseless_isequal(
                            name_intern.clone(),
                            self.propstrings[string_index as uint].clone()
                        )
                    ) {
                    return_value = Some((CSS_SELECTOR_PSEUDO_CLASS, string_index));
                }
            }

            for self.pseudo_element_list.each |&string_index| {
                if (
                    l.lwc_string_caseless_isequal(
                        name_intern.clone(), 
                        self.propstrings[string_index as uint].clone()
                    )
                ) {
                    return_value = Some((CSS_SELECTOR_PSEUDO_ELEMENT , string_index));
                }
            }
        }

        return_value
    }
}

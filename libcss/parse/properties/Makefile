all:
	rm -f autogenerated.rs
	rustc css_property_parser_gen.rs -o css_properties_parser_gen
	
debug:
	rm -f autogenerated.rs
	rustc -Z verbose -Z debug-info css_property_parser_gen.rs -o css_properties_parser_gen

run:
	./css_properties_parser_gen

clean:
	rm -f css_properties_parser_gen
	rm -f autogenerated.rs

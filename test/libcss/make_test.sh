# do clean
rm -rf *.so *.o
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet css21.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse-auto.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse2-auto.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet lex.rs -L .
rustc --test -L ../../libparserutils/ csdetect.rs -L .
##$ global.iter = 4;
###
##$ // Generate multiple command
##$ for (let i = 0; i < global.iter; i++) {
##$     println(`say ${i}`);
##$ }
say 0
say 1
say 2
say 3
###
##$ // Generate single command
##$
##$ print(`execute`)
##$ for (let i = 0; i < 10; i++) {
##$     print(`if block ~ ~${i} ~ stone`);
##$ }
##$ print(`run say hi`)
execute if block ~ ~0 ~ stone if block ~ ~1 ~ stone if block ~ ~2 ~ stone if block ~ ~3 ~ stone if block ~ ~4 ~ stone if block ~ ~5 ~ stone if block ~ ~6 ~ stone if block ~ ~7 ~ stone if block ~ ~8 ~ stone if block ~ ~9 ~ stone run say hi
###

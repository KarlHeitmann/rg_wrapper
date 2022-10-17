# OVERVIEW

This is a [ripgrep](https://github.com/BurntSushi/ripgrep) wrapper. The idea is to have a main loop in which the program
will ask you an option, and then it runs ripgrep with `--json` flag, and the wrapper can take the output of ripgrep and
do other things with the output.

It can be pretty useful to explore an unknown source code. You can navigate the code by greping something, and do some
sort of "nested greps" inside the results

# DEPENDENCIES

You must install [ripgrep](https://github.com/BurntSushi/ripgrep) to run this program, because `rg_wrapper` will launch
an instance of `ripgrep` each time it is used. To check if you have `ripgrep installed` type in the following command on
your terminal and you should see a biiiig help message.

> rg --help




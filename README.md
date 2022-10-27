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

```bash
rg --help
``` 

You should see something starting with this, and you're ready to go:

> ripgrep 13.0.0
> Andrew Gallant <jamslam@gmail.com>
> 
> ripgrep (rg) recursively searches the current directory for a regex pattern.
> By default, ripgrep will respect gitignore rules and automatically skip hidden
> files/directories and binary files.

If you see something like "error" when typing the `rg --help` command, then you need to install ripgrep or found out what
did happened with your rip grep installation (if you're running windows, did you install ripgrep in the same shell you are
running this project? eg: bash shell, powershell, wsl, etc)

# RELATED PROJECTS

[rg_wrapper_text_analysis](https://github.com/KarlHeitmann/rg_wrapper_text_analysis)



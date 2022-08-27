# How to make an aesthetic cse terminal

From

![Image](https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/basic-terminal.png)

to

![Image](https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/pretty-terminal.png)

## Some basic colours :)

If you're on a UNSW CSE server, you can run the command

```bash
1511 colours
```

it should setup your bash terminal to now be colourful! 
Make sure to refresh your terminal configuration - either by closing and opening, or by running the command: 

```bash
source ~/.bashrc
```

in order to see it take effect.

![Image](https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/1511-colours.png)

## Powerlevel10k

### What is powerlevel10k?

Powerlevel10k is one of the most popular plugin frameworks for Zsh. It comes with many features and extensions, and allows you to customise nearly everything to your liking. It brags an "out of box" experience, meaning that theres not much setup we have to do get it working :D

![Image](https://raw.githubusercontent.com/romkatv/powerlevel10k-media/master/prompt-styles-high-contrast.png)

### What is zsh?

You may be thinking, hold up - what is zsh?

let's take a step back - a terminal is a program that runs a shell - so when you open your terminal in Vlab or elsewhere, a shell is automatically opened, and executed. On CSE Servers, the default login shell is `bash`.

ZSH (aka the Z Shell) - is nothing more than a different shell. So why do we care if we use `bash` vs `zsh` as our shell? What's the difference?

### bash vs zsh

The difference mainly between zsh and bash is that zsh comes with an extended set of features, and many well supported plugin and theme managers, such as powerlevel10k - which we mentioned above. Some of the useful productivity hacks include:

- command history shared across all running shells, rather than just the current running shell
- better tab completion (zsh cycles through all options, rather than listing them all)
- spelling correction and autofill
- Directory aliases (such as `~` or `..` - you can make your own)
- git command completion and aliases
- path expansionn e.g. Enter cd /u/lo/b, press tab, and it will be completed to cd /usr/local/bin since it is the only matching pattern


Most importantly to us - it allows us to make our terminal ✨LOOK PRETTY ✨!

## Installing zsh

If you're on cse - you don't need to do this! If you want to change
your shell to permanently be `zsh` - you should email ss@cse.unsw.edu.au
to get it changed! 

Otherwise, you can run the `zsh` command on each login (or find an automated way to 
do this! 😉)

If you're looking to install `zsh` on your local machine, you can view the
setup instructions [here](https://github.com/ohmyzsh/ohmyzsh/wiki/Installing-ZSH).

## Installing Powerlevel10k

### Installing ohmyzsh

ohmyzsh is a plugin manager for managing zsh configurations. We'll use this to install powerlevel10k soon.

The default location is `~/.oh-my-zsh` (hidden in your home directory, you can access it with `cd ~/.oh-my-zsh`)

To install ohmyzsh - follow the instructions [here](https://github.com/ohmyzsh/ohmyzsh#getting-started), or trust that I'm not trying to hack your computer, and run the following:

```bash
sh -c "$(wget -O- https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

This will install and setup a default zsh theme, and put some sane presets in `~/.zshrc`

### Installing powerlevel10k

Now that we've got ohmyzsh, we want to install the powerlevel10k theme.

Take a look at the instructions [here](https://github.com/romkatv/powerlevel10k#oh-my-zsh), or trust me again (???) and run

```bash
git clone --depth=1 https://gitee.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
```

and then finally, we set the theme (should be on line 11) in our `~/.zshrc`:

```txt
ZSH_THEME="powerlevel10k/powerlevel10k"
```
We also want to add the following line to our zshrc file (preferably the end!)

```txt
export TERM=xterm-256color
```

Please also install the correct fonts (if you're on CSE, scroll down!) by following the instructions [here](https://github.com/romkatv/powerlevel10k/blob/master/README.md#fonts)

after you save this, we need to once again make sure to refresh your terminal configuration with

```bash
source ~/.zshrc
```

in order to see it take effect. 

If nothing happens, run: 
```
zsh
```

Then, to make sure that we run OUR version of zsh every time, we add the folllowing to the TOP (!!) of `~/.bashrc`

```
export SHELL="/usr/local/bin/zsh";
exec "$SHELL" -l;
```

Finally, run

```bash
source ~/.bashrc
```

and you'll be met with the Powerlevel10k configuration wizard (YAY!)

Go through and answer the questions, formatting your terminal to your liking.

Congratulations you now have a pretty terminal!


# Further extensions: plugins!

## zsh-syntax-highlighting

Clone the syntax highlighter

```bash
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
```

Activate it by adding it to the `plugins` array inside `~/.zshrc`, and then sourcing `zshrc`

ie making sure that the plugins line is:

```txt
plugins=(git zsh-syntax-highlighting)
```

and then running

```bash
source ~/.zshrc
```

## zsh-autosuggestions

Suggesting commands as you type based on history and completions!!
As you type commands, you will see a completion offered after the cursor in a muted gray color.
If you press the<kbd> → </kbd> key (forward-char widget) or <kbd>End</kbd> (end-of-line widget) with the cursor at the end of the buffer, it will accept the suggestion, replacing the contents of the command line buffer with the suggestion.

Clone the plugin

```bash
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
```

Activate it by adding it to the `plugins` array inside `~/.zshrc`, and then sourcing `zshrc`

ie making sure that the plugins line is:

```txt
plugins=(git zsh-syntax-highlighting zsh-autosuggestions)

```

and then running

```bash
source ~/.zshrc
```

## Zsh-z

ZSH-z is a command line tool that allows you to jump quickly to directories that you have visited frequently in the past, or recently -- but most often a combination of the two (a concept known as "frecency"). It works by keeping track of when you go to directories and how much time you spend in them.

Same as the other plugins, we clone

```bash
git clone https://github.com/agkozak/zsh-z $ZSH_CUSTOM/plugins/zsh-z
```

add

```bash
plugins=(git zsh-syntax-highlighting zsh-autosuggestions zsh-z)
```

and update

```
source ~/.zshrc
```

## Exa

Exa is a "modern replacement for _ls_
![Image](https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/exa.png)

simply run

```bash
cargo install exa
mv ~/.cargo/bin/exa ~/bin
```

you can remove cargo if you are running out of space, since we have kept exa in our own bin.

I like to use the alias

```bash
alias ll="exa -l"
alias l="exa -lah"
```

in my zshrc file.

# Conclusion

You now have a pretty terminal, and theres so much more you can do!

Checkout the following extra resources

# DLC 

#### Installing fonts

Download the four font files on the powerlevel10k homepage, then inside vlab, create a folder in your home directory (~) `mkdir ~/.fonts` and move all the font files there. 
Open xterm and go edit->preferences->appearances  and select the installed font. 

### Further Customisation:

Powerlevel10k configuration: [https://github.com/romkatv/powerlevel10k/blob/master/README.md#configuration](https://github.com/romkatv/powerlevel10k/blob/master/README.md#configuration)

Prompt colour customisation: [https://github.com/romkatv/powerlevel10k/blob/master/README.md#how-do-i-change-prompt-colors](https://github.com/romkatv/powerlevel10k/blob/master/README.md#how-do-i-change-prompt-colors)

### Fonts are showing weird symbols

Install the correct font into your terminal!

Works on CSE through ssh, since obviously the font being used is on your local machine.

Take a look at this section: [https://github.com/romkatv/powerlevel10k#fonts](https://github.com/romkatv/powerlevel10k#fonts) of the readme.

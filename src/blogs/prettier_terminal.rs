use yew::prelude::*;

pub fn content() -> Html {

    html! {
        <>
        <h1 style="text-align: center;">{"How to make an aesthetic terminal"}</h1>
        <p>{"From"}</p>
        <img style="text-align: center; max-width: 100%;" src="https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/basic-terminal.png"/>
        <p>{"to"}</p>
        <img style="text-align: center; max-width: 100%;" src="https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/pretty-terminal.png"/>
        
        <h2 id="some-basic-colours-">{"Some basic colours :)"}</h2>
        <p>{"So, you want some colours eh?"}</p>
        <p>{"Simply run the command"}</p>
        <pre>
            <code class="lang-bash">
                <span class="hljs-symbol">{"1511"}</span> {"colours"}
            </code>
        </pre>
        <p>{"should setup your bash terminal to now be colourful! Make sure to refresh your terminal configuration with"}</p>
        <pre>
            <code class="lang-bash">
                <span class="hljs-built_in">{"source"}</span> {"~/.bashrc"}
            </code>
        </pre>
        <p>{"in order to see it take effect."}</p>
        <p><img style="margin: 0 auto; max-width: 100%;" src="https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/1511-colours.png"/></p>
        <h2 id="powerlevel10k">{"Powerlevel10k"}</h2>
        <h3 id="what-is-powerlevel10k-">{"What is powerlevel10k?"}</h3>
        <p>{"Powerlevel10k is one of the most popular plugin frameworks for Zsh. It comes with many features and extensions, and allows you to customise nearly everything to your liking. It brags an &quot;out of box&quot; experience, meaning that theres not much setup we have to do get it working :D"}</p>
        <p><img style="margin: 0 auto; max-width: 100%;" src="https://raw.githubusercontent.com/romkatv/powerlevel10k-media/master/prompt-styles-high-contrast.png"/></p>
        <h3 id="what-is-zsh-">{"What is zsh?"}</h3>
        <p>{"You may be thinking, hold up - but what actually is zsh?"}</p>
        <p>{"let's take a step back - a terminal is a program that runs a shell - so when you open your terminal in Vlab or elsewhere, a shell is automatically opened, and executed. On UNSW CSE Servers, the default login shell is "}<code>{"bash"}</code>{"."}</p>
        <p>{"ZSH (aka the Z Shell) - is nothing more than a different shell. So why do we care if we use "}<code>{"bash"}</code>{" vs "}<code>{"zsh"}</code>{" as our shell? What's the difference?"}</p>
        <h3 id="bash-vs-zsh">{"bash vs zsh"}</h3>
        <p>{"The difference mainly between zsh and bash is that zsh comes with an extended set of features, and many well supported plugin and theme managers, such as powerlevel10k - which we mentioned above. Some of the useful productivity hacks include:"}</p>
        <ul>
        <li>{"command history shared across all running shells, rather than just the current running shell"}</li>
        <li>{"better tab completion (zsh cycles through all options, rather than listing them all)"}</li>
        <li>{"spelling correction and autofill"}</li>
        <li>{"Directory aliases (just like "}<code>{"~"}</code>{" or "}<code>{".."}</code> {"- you can make your own)"} </li>
        <li>{"git command completion and aliases"}</li>
        <li>{"path expansionn e.g. Enter cd /u/lo/b, press tab, and it will be completed to cd /usr/local/bin since it is the only matching pattern"}</li>
        </ul>
        <p>{"Most importantly to us - it allows us to make our terminal LOOK PRETTY"}</p>
        <h2 id="installing-zsh-on-cse">{"Installing zsh on UNSW CSE"}</h2>
        <p>{"Back in the old days, you had to maintain your own instalation, but I emaled SS and they were nice enough to install a global updated version for us!"}</p>
        <h2 id="installing-powerlevel10k">{"Installing Powerlevel10k"}</h2>
        <h3 id="installing-ohmyzsh">{"Installing ohmyzsh"}</h3>
        <p>{"ohmyzsh is a plugin manager for managing zsh configurations. We'll use this to install powerlevel10k soon."}</p>
        <p>{"The default location is "}<code>{"~/.oh-my-zsh"}</code>{" (hidden in your home directory, you can access it with "}<code>{"cd ~/.oh-my-zsh"}</code>{")"}</p>
        <p>{"To install ohmyzsh - follow the instructions "}<a href="https://github.com/ohmyzsh/ohmyzsh#getting-started">{"here"}</a>{", or trust that I'm not trying to hack your computer, and run the following:"}</p>
        <pre>
            <code class="lang-bash">{"sh -c "}<span class="hljs-string">{"\""}<span class="hljs-variable">{"$(wget -O- https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"}</span>{"\""}</span>
            </code>
        </pre>
        <p>{"This will install and setup a default zsh theme, and put some sane presets in "}<code>{"~/.zshrc"}</code></p>
        <h3 id="installing-powerlevel10k">{"Installing powerlevel10k"}</h3>
        <p>{"Now that we&#39;ve got ohmyzsh, we want to install the powerlevel10k theme."}</p>
        <p>{"Take a look at the instructions "}<a href="https://github.com/romkatv/powerlevel10k#oh-my-zsh">{"here"}</a>{", or trust me again (???) and run"}</p>
        <pre>
            <code class="lang-bash">{"git clone --depth="}<span class="hljs-number">{"1"}</span>{" https:"}<span class="hljs-regexp">{"//gi"}</span>{"tee.com"}<span class="hljs-regexp">{"/romkatv/"}</span>{"powerlevel10k.git "}<span class="hljs-variable">{"${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}"}</span><span class="hljs-regexp">{"/themes/"}</span>{"powerlevel10k"}
            </code>
        </pre>
        <p>{"and then finally, we set the theme (should be on line 11) in our "}<code>{"~/.zshrc</code>:"}</code></p>
        <pre>
            <code class="lang-txt"><span class="hljs-attr">{"ZSH_THEME"}</span>{"="}<span class="hljs-string">{"\"powerlevel10k/powerlevel10k\""}</span>
            </code>
        </pre>
        <p>{"We also want to add the following line to our zshrc file (preferably the end!)"}</p>
        <pre>
            <code class="lang-txt"><span class="hljs-keyword">{"export"}</span>{"TERM=xterm"}<span class="hljs-number">{"-256"}</span><span class="hljs-built_in">{"color"}</span>
            </code>
        </pre>
        <p>{"Please also install the correct fonts (if you're on CSE, scroll down!) by following the instructions "}
            <a href="https://github.com/romkatv/powerlevel10k/blob/master/README.md#fonts">{"here"}</a>
        </p>
        <p>{"after you save this, we need to once again make sure to refresh your terminal configuration with"}</p>
        <pre>
            <code class="lang-bash"><span class="hljs-built_in">{"source"}</span>{" ~/.zshrc"}
            </code>
        </pre>
        <p>{"in order to see it take effect. "}</p>
        <p>{"If nothing happens, run: "}</p>
        <pre><code><span class="hljs-attribute">{"zsh"}</span>
        </code></pre><p>{"or "}<code>{"exec zsh"}</code>
        {"(thanks to @AutumnalBlake for pointing this out)"}</p>
        <p>{"Then, to make sure that we run OUR version of zsh every time, we add the folllowing to the TOP (!!) of "}<code>{"~/.bashrc"}</code></p>

        <pre>
            <code><span class="hljs-built_in">{"export"}</span>{" SHELL="}<span class="hljs-string">{"\"/usr/local/bin/zsh\";"}</span>
            <span class="hljs-built_in">{"exec"}</span>{" "}<span class="hljs-string">{"\""}<span class="hljs-variable">{"$SHELL"}</span>{"\""}</span>{" "}<span class="hljs-_">{"-l"}</span>{";"}
            </code>
        </pre>

        <p>{"Finally, run"}</p>
        
        <pre>
            <code class="lang-bash"><span class="hljs-built_in">{"source"}</span>{" ~/.bashrc"}
            </code>
            </pre>
        <p>{"and you'll be met with the Powerlevel10k configuration wizard (YAY!)"}</p>
        <p>{"Go through and answer the questions, formatting your terminal to your liking."}</p>
        <p>{"Congratulations you now have a pretty terminal!"}</p>
        <h2 id="further-extensions-customisation">{"Further extensions + customisation"}</h2>
        <h3 id="plugins-">{"Plugins!"}</h3>
        <h4 id="zsh-syntax-highlighting">{"zsh-syntax-highlighting"}</h4>
        <p>{"Clone the syntax highlighter"}</p>
        <pre><code class="lang-bash">{"git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting"}
        </code></pre>
        <p>{"Activate it by adding it to the "}<code>{"plugins"}</code>{" array inside "}<code>{"~/.zshrc"}</code>{", and then sourcing "}<code>{"zshrc"}</code></p>
        <p>{"ie making sure that the plugins line is:"}</p>
        <pre><code class="lang-txt">{"plugins=(git zsh-syntax-highlighting)"}
        </code></pre>
        <p>{"and then running"}</p>
        <pre><code class="lang-bash"><span class="hljs-built_in">{"source"}</span>{" ~/.zshrc"}
        </code></pre>
        <h4 id="zsh-autosuggestions">{"zsh-autosuggestions"}</h4>
        <p>{"Suggesting commands as you type based on history and completions!!
        As you type commands, you will see a completion offered after the cursor in a muted gray color.
        If you press the"}<kbd>{" → "}</kbd>{" key (forward-char widget) or "}<kbd>{"End"}</kbd>{" (end-of-line widget) with the cursor at the end of the buffer, it will accept the suggestion, replacing the contents of the command line buffer with the suggestion."}</p>
        <p>{"Clone the plugin"}</p>
        <pre><code class="lang-bash">{"git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions"}
        </code></pre>
        <p>{"Activate it by adding it to the plugins array inside "}<code>{"~/.zshrc"}</code>{", and then sourcing your zshrc"}</p>
        <p>{"ie making sure that the plugins line is:"}</p>
        <pre>
            <code class="">
            <span class="">{"plugins"}</span>{"=(git zsh-syntax-highlighting zsh-autosuggestions)"}
            </code>
        </pre>
        <p>{"and then running"}</p>
        <pre>
            <code>{" source ~/.zshrc"}
            </code>
        </pre>
        <h4 id="zsh-z">{"zsh-z"}</h4>
        <p>{"ZSH-z is a command line tool that allows you to jump quickly to directories that you have visited frequently in the past, or recently -- but most often a combination of the two (a concept known as 'frecency'). It works by keeping track of when you go to directories and how much time you spend in them."}</p>
        <p>{"Same as the other plugins, we clone"}</p>
        <pre>
            <code class="lang-bash">
                {"git clone https://gi</span>thub.com/agkozak/zsh-z $ZSH_CUSTOM/plugins/zsh-z"}   
            </code>
        </pre>
        <p>{"add"}</p>
        <pre>
            <code class="lang-bash">
                <span class="hljs-attr">{"plugins"}</span>{"=(git zsh-syntax-highlighting zsh-autosuggestions zsh-z)"}
            </code>
        </pre>
        <p>{"and update"}</p>
        <pre>
            <code>
                <span class="hljs-built_in">{"source"}</span>{" ~/.zshrc"}
            </code>
        </pre>

        <h3 id="exa">{"exa"}</h3>
        <p>{"Exa is a modern replacement for ls"}
        <img style="margin: 0 auto;max-width: 100%; " src="https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/assets/images/exa.png" alt="exa picture"/></p>
        <p>{"simply run"}</p>
        <pre>
            <code class="lang-bash">
                {"cargo install exa"}
                {"mv ~/.cargo/bin/exa ~/bin"}
            </code>
        </pre>
        <p>{"you can remove cargo if you are running out of space, since we have kept exa in our own bin."}</p>
        <p>{"I like to use the alias"}</p>
        <pre>
            <code class="lang-bash"><span class="hljs-keyword">{"alias"}</span>{" "}<span class="hljs-title">{"ll"}</span>{"="}<span class="hljs-string">{"\"exa -l\""}</span>
            </code>
        </pre>
        <p>{"in my `~/.zshrc`"}</p>
        <h1 id="conclusion">{"Conclusion"}</h1>
        <p>{"You now have a pretty terminal, and theres so much more you can do!"}</p>
        <p>{"Checkout the following extra resources"}</p>
        <h1 id="extras-notes">{"Extras + Notes"}</h1>
        <h3 id="running-on-cse">{"running on CSE"}</h3>
        <h4 id="installing-fonts">{"installing fonts"}</h4>
        <p>{"Download the four font files on the powerlevel10k homepage, then inside vlab, create a folder in your home directory (~) <code>mkdir ~/.fonts</code> and move all the font files there. 
        Open xterm and go edit->preferences->appearances  and select the installed font."}</p>
        <h4 id="fetching-gitstatusd">{"fetching gitstatusd"}</h4>
        <p>{"Every time you login (/open) a cse terminal - you might see 'fetching gitstatusd...' for a few seconds.
        While this probably wouldnt annoy the average person - it absolutely infuriates me (and a few others)/"}</p>
        <p><a href="https://github.com/insou22">{"@insou22"}</a>{" has kindly provided a script to fix this issue, which you can run using"}</p>
        <pre>
            <code class="lang-bash">{"curl "}<span class="hljs-string">{"https://gist.insou.dev/gitstatus | sh"}</span>
            </code>
        </pre>
        <p>{"followed by"}</p>
        <pre>
            <code class="lang-bash"><span class="hljs-built_in">{"source"}</span>{" ~/.bashrc"}
            </code>
        </pre>
        <p>{"This will download and ensure that every time we open zsh - the computer knows where to find it, wheras whats happenning currently is that the cache is wiped every time you log off which is #notgood"}</p>
        <h3 id="further-customisation-">{"Further Customisation:"}</h3>
        <p><a href="https://github.com/romkatv/powerlevel10k/blob/master/README.md#configuration">{"powerlevel10k configuration"}</a></p>
        <p><a href="https://github.com/romkatv/powerlevel10k/blob/master/README.md#how-do-i-change-prompt-colors">{"prompt colour customisation"}</a></p>
        <h3 id="fonts-are-showing-weird-symbols">{"Fonts are showing weird symbols"}</h3>
        <p>{"Install the correct font into your terminal - I&#39;m still yet to get this working on VLAB."}</p>
        <p>{"Works on CSE through ssh, since obviously the font being used is on your local machine."}</p>
        <p>{"Take a look at "}<a href="https://github.com/romkatv/powerlevel10k#fonts">{"https://github.com/romkatv/powerlevel10k#fonts"}</a></p>
        </>
    }
    
}

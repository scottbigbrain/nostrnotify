# nostrnotify
`nostrnotify` is a tool for automatically posting updates on podcast rss feeds to nostr. It can post notifications of new episodes, live streams, and possibly more to come! :)

**NOTE: This project is still in alpha and should not be used unless you are helping to develop it**

# Installation
At the moment, `nostrnotify` is in development and does not have any released binaries. But, if you want to try using it or building on it, you can easily compile from source.
```
~$ git clone https://github.com/scottbigbrain/nostrnotify
~$ cd nostrnotify
~/nostrnotify/$ cargo build
```

# Setup
## Generating Keys
After installing the binary, you will want to setup a pair of nostr keys. If you have a secret key already, you can use that. Note that `nostrnotify` assumes that this nostr identity belongs to the bot alone and won't be consistantly used for anything else, so don't use the key for your personal account.
```
~$ nostrnotify secret-key nsec10krt7wu8eltpjrlh0u5f26g72fy3j90w023lq3z0tv8lt9qlaqsqftex29
```
**NOTE: DO NOT USE THIS SECRET KEY IT IS FOR DEMONSTRATION PURPOSES ONLY**
If you want to generate brand new keys for the bot, run the appropriate command and choose to store the keys to config.
```
~$ nostrnotify generate-keys
Keys Generated
Public Key: npub1vgrhf4d0xlq52ztzn4cjjsr98pn7g7xz5jt83qd852ypqslpp0dqjn8ze5
Private Key: nsec10krt7wu8eltpjrlh0u5f26g72fy3j90w023lq3z0tv8lt9qlaqsqftex29
Do you want to store the new keys to config? This will overwrite the currently stored key. yes
```
## Giving the Bot a Name and Description
Now, set a username, display name, and description for your bot. The description is optional.
```
~$ nostrnotify set-metadata
Enter bot username: johndoebot
Enter bot display name: John Doe Bot
Do you want to include a description? no
```
## Monitoring a Feed
To configure the feed you want to monitor, just run
```
~$ nostrnotify feed-url <feed url>
```
## Setting an Interval
`nostrnotify` will only download the feed every so often in order to balance quick updates with bandwidtch contraints. The default for this is 300 seconds (checks feed every 5 minutes). You can set a custom interval with
```
~$ nostrnotify interval <interval in seconds>
```
## Adding Relays
`nostrnotify` automatically connects you to the free relays of `wss://nos.lol/` and `wss://house.relay/`. You can add your own relays, although paid relays are not yet supported.
```
~$ nostrnotify add-relay <relay uri>
```

# Usage
To start monitoring the feed, just run 
```
~$ nostrnotify run
```
Currently, this will just run in the terminal and log updates to the console. If you don't want it flooding the terminal then run it as a backgroun process.

# Contributing
I am currently working on this alone, but I am very open to anyone joining me in the development. There is not yet a specific means of contributing, but feel free to open up issues and PRs and we can talk on there.

There is a list of features that you can work on developing if you want.

There is a test bot I have been pushing test notifications to that you can look at if you want. 
npub1mp2dt4gnfvull2l8wnc92dzmxkq9rkq982qhq47s6jtv9l6l9d3scfp9uk

# TODO
[ ] Include links in notifications
[ ] Allow for one instance pushing notifications for multiple feeds
[ ] Write tests :/
[ ] Fix time formatting on live stream notifications
[ ] Make notifications easier to read

> "He that hath an ear, let him hear what the Spirit saith unto the
> churches; To him that overcometh will I give to eat of the hidden
> manna, and will give him a white stone, and in the stone a new name
> written, which no man knoweth saving he that receiveth it."
>
> Apocalypse 2:17

# Eschanostr

Convert electricity into read friendly nostr npub.

## Install

```sh
sudo wget -O /usr/bin/eschanostr https://github.com/antonioconselheiro/eschanostr/releases/download/1.0.1/eschanostr;
sudo chmod +x /usr/bin/eschanostr
```

Or just download the last version in releases page.

## Running

**Basic run:**

`eschanostr --nregex "deep" --npassword "jesusteama"`

or

`eschanostr -r "h0ney" -p "reijesus"`

**Complex regexes:**

`eschanostr --nregex ".*pl[a4]c[e3]" --npassword "jesusteama"`

You can compose your regex using [this tool](https://jex.im/regulex/#!flags=&re=.*h%5Bo0%5Drs%5Be3%5Ds%3F).

The algorithm use brute-force search, so if you should fill the regex with all your read-friendly npub expectations, this way you can do it only one time.

**Dev run**

`cargo run -- --nregex "sh0p" --npassword "jesuslindo"`

See more in [CONTRIBUTE.md](./CONTRIBUTE.md).

## Approximate time

The lower the requirement, the faster the npub friendly readable will be found, the higher the requirement, the slower it will be found (except if you dance).

- One or two letter - less than one second (20ms faster if you dance)
- Three or four letter - less than ten seconds
- Five letters - less than five minutes
- Six letters - can make some in a hour
- Seven letters - some time, I never tested this or more

You can find it faster if you search for a pattern inside the npub instead of in the begining (by starting your regex with `.\*`) and dance on run.

## How it works?

<pre>
Basically this nguys dances
until find a nsec
that derivate into a npub
that matches with your nregex
then print a ncryptsec
that you can open using you npassword
</pre>

![Animated GIF of Dr Neo Cortex from Crash Bandicoot series dancing russian dance kazotsky](./dr-neo-cortex-kazotsky.gif)

## Donate

Lighting donate: <a href="lightning:antonioconselheiro@getalby.com">lightning:antonioconselheiro@getalby.com</a>

![zap me](https://raw.githubusercontent.com/antonioconselheiro/antonioconselheiro/main/img/qrcode-wallet-lighting.png)

Bitcoin onchain donate: <a href="bitcoin:bc1qrm99lmmpwk7zsh7njpgthw87yvdm38j2lzpq7q">bc1qrm99lmmpwk7zsh7njpgthw87yvdm38j2lzpq7q</a>

![zap me](https://raw.githubusercontent.com/antonioconselheiro/antonioconselheiro/main/img/qrcode-wallet-bitcoin.png)

---
title: "Making an RSS & system notifier for research papers in haskell"
date: "2026-09-18"
tags: ["haskell", "fp"]
wordNum: 0
readTime: 0
---

## Haskell trouble

I haven't previously worked with a functional programming language,
so I decided it was time to finally pick one up after hearing
for years that Haskell is very unique compared to other languages
and provides a new way of thinking about systems. I will agree
with most points except the last one (as it's still too new
for me to agree with). 

It feels similar to Rust in some regards, as sometimes I also
use rust as a language thats missing OOP. Example for structs:

```hs
data Article = Article
  { link :: String
  , pubDate :: String
  , title :: String
  }
```

Unlike Rust though, I feel that Haskell suffers in terms of
documentation. I have yet to find a proper document that goes
through even it's syntax. The only official doc I could find
is one that is provided as a manual in case you want to make
a haskell compiler, which is obviously not what I am looking
for.

## Cool things about haskell so far

I went in with no expectation of the language, as I had
never even seen the syntax of the language. Initially,
the syntax looked malformed, odd and something I never
wanted to see again after the first couple of hours. 
Surprisingly I can appreciate how function declarations
and definitions work together, you name a type and then
name the variable in the next line (potentially ever
avoiding having a massive line, or breaking a function
declaration into multiple lines to fit the params). The
package manager seems a bit finicky sometimes, but seems
to work for the most part. The language server feels like
a treat after using some horrendous things in other
languages. Additionally there is a lack of visibility
keywords (obviously), which can be implemented via
what you explicitly want to export from your module,
otherwise everything is static function (c lang) /
private by default.

## Program
Zotero was one of the applications I used extensively
when trying to build a journal of documents I had
read and will read in the future. However, I didn't
appreciate how heavy weight the app would become
once the library builds up. So, since I never
used Zotero as a pdf reader, I could simply make my
own app that keeps track of what I have read,
will read and what I am interested in reading.

Simply put I am just making an RSS feeder
and HTML extractor based on journals that are
peer reviewed and some that aren't previously
reviewed. This will then keep track of what has 
previously been received and notify the system
when a new interesting paper comes out. I simply
hate going through some of the websites where
these research papers are hosted, and have to
go through a bunch of useless papers before I 
can find something interesting. This saves me
some time and also keeps me always reading
something in case I have some free time on my hand.

## Results so far
I find it quite interesting that on pure recursion 
and no mutability the program is somehow still performant.

This is what the results of the current project look like.

```txt
Link: https://onlinelibrary.wiley.com/doi/10.1111/mafi.70061?af=R
Date: Thu, 17 Sep 2026 21:05:05 -0700
Title: Iterative Schemes for Markov Perfect Equilibria
---
Link: https://onlinelibrary.wiley.com/doi/10.1111/mafi.70056?af=R
Date: Mon, 14 Sep 2026 22:45:27 -0700
Title: Sensitivity Analysis of Distributionally Robust BSDEs and RBSDEs
---
Link: https://onlinelibrary.wiley.com/doi/10.1111/mafi.70059?af=R
Date: Tue, 08 Sep 2026 07:34:36 -0700
Title: Climate and Nature Risk in Mathematical Finance
---
Link: https://onlinelibrary.wiley.com/doi/10.1111/mafi.70055?af=R
Date: Tue, 08 Sep 2026 07:34:36 -0700
Title: Issue Information
---
Link: https://onlinelibrary.wiley.com/doi/10.1111/mafi.70029?af=R
Date: Tue, 08 Sep 2026 07:34:36 -0700
Title: Navigating Supply Shocks: Sector Resilience and Production Prices Through Stochastic Input–Output Modeling
---
```

## Upgrades to the system
Everything works fine now for the most part, I get notified 
and the RSS feeds are parsed properly. Everytime my system is 
notified I can click the notification and it redirects me to
the paper. This is good and all but I would rather my gaming pc
not use 100MB constantly and also every hour have to go through a
massive spike in TCP packets. So the idea now is, since I already
have a homeserver, I can set up a centralized notifier
running constantly in my homeserver which acts as a publisher for
all clients connected to this socket. I will be using MQTT
for this. 

### MQTT-3.1.1 Client
Following the standard of mqtt [MQTT 3.1.1](https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html)
I have this struct which encapsualtes a full package being sent
or received. The struct is pretty simplistic, as it's necessary
to be performant. Generally fixed header is almost never 5 bytes,
but instead of heap allocating vectors i just decide to hold
5 bytes in an array and then during encoding I figure out
what byte should be sent or not.

```rs
#[derive(Debug)]
pub struct Message {
    fixed_header: [u8; 5],
    var_header: Vec<u8>,
    payload: Vec<u8>,
    state: State,
}
```

The encoding to bytes is shown below together with how to 
encode the length of the whole payload (variable header + 
payload length, as specified by the standard). Encoding 
using variable length encoding algorithm, and through it
we can represent a package length up to 268M bytes (~260MB).

```rs
pub fn to_bytes(&self) -> Vec<u8> {
    let payload_len = encode_length(self.var_header.len() + self.payload.len());
    let msg_len = 1 + payload_len.1 + self.var_header.len() + self.payload.len();

    let mut bytes = Vec::with_capacity(msg_len);
    bytes.push(self.fixed_header[0]);
    bytes.extend_from_slice(&payload_len.0[0..payload_len.1]);
    bytes.extend_from_slice(&self.var_header);
    bytes.extend_from_slice(&self.payload);
    bytes
}

fn encode_length(mut x: usize) -> ([u8; 4], usize) {
    let mut bytes = [0u8; 4];
    let mut count = 0;

    loop {
        let mut encoded_byte = (x % 128) as u8;
        x /= 128;

        if x > 0 {
            encoded_byte |= 128;
        }
        bytes[count] = encoded_byte;
        count += 1;
        if x == 0 {
            break;
        }
    }

    (bytes, count)
}

pub fn decode_length(data: &[u8]) -> u32 {
    let mut multiplier: u32 = 1;
    let mut value = 0;
    let mut index = 0;
    let mut encoded_byte = 0;

    loop {
        encoded_byte = data[index];
        value += (encoded_byte as u32 & 127) * multiplier;
        multiplier *= 128;
        if multiplier > 128 * 128 * 128 {
            panic!("Something definitely wrong here chief");
        }

        if (encoded_byte & 128) == 0 {
            break;
        }
        index += 1;
    }

    value
}
```

By the standard to fully have a client that passes all checkmarks you
need a client that is able to communicate through streaming
and through websocket. Currently my client only does TCP and no
websocket(shouldn't be too difficult to implement and I will in the future).
But the way to receive the full payload through tcp is my simply getting
the first byte, which represents the flags and the type of packet request
that's coming from the server. Then we parse the second byte, which lets us
know how many bytes we should wait from the streaming packet (remaining bytes).
This way we know for sure we will always receive the full packet and no extra
or less bytes. Method for receiving is shown below.

```rs
pub fn receive(&mut self) -> Result<Option<Message>, std::io::Error> {
    let mut header_byte = [0u8; 1];
    let mut len_bytes = Vec::with_capacity(4);
    let mut body = Vec::new();

    {
        let stream = match self.client.as_mut() {
            Some(s) => s,
            None => return Ok(None),
        };

        match stream.read(&mut header_byte) {
            Ok(0) => {
                self.client = None;
                return Ok(None);
            }
            Ok(_) => {}
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                return Ok(None);
            }
            Err(e) => return Err(e),
        }

        let mut single_byte = [0u8; 1];
        loop {
            stream.read_exact(&mut single_byte)?;
            len_bytes.push(single_byte[0]);
            if (single_byte[0] & 128) == 0 {
                break;
            }
        }

        let remaining_len = decode_length(&len_bytes) as usize;
        if remaining_len > 0 {
            body.resize(remaining_len, 0);
            stream.read_exact(&mut body)?;
        }
    }
    // ...
```

## Crate and repository
For any updates or if a want to use a simple mqtt client with no
external dependencies arises, both links can be found here.

[Rust crate](https://crates.io/crates/zabamqtt)

[Repository hosted in GitHub](https://github.com/Skyfrei/zabamqtt)


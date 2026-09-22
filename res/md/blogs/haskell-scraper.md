---
title: "Making a scrapper & system notifier for research papers in haskell"
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



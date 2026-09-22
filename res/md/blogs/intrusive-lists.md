---
title: "Using intrusive linked lists in order book"
date: "2026-07-01"
tags: ["C++", "Trading"]
wordNum: 0
readTime: 0
---


## Working with order books

Generally these order books are split into
levels ranging from L1, L2, L3 and sometimes order
books which aren't accessible to normal people but
only institutes.

Typically order books also split the market
into different sections as well, based on how an order
is being placed. Some order can be market orders which buys
or sells an asset immediately. Some other orders wait until
it reaches your destined price etc. This blog for the time
being won't look at all these (as its still a WIP) but mainly
just look at implementation of making the order books
performative.

## Ways of implementing order books

Some of these implementations were tested in real trading,
although not tested extensively, but from what little 
real trading was done with them they looked to function
properly.
My first initial implementation of an order book was
plainly using a linked list (yes...). 
```cpp
std::map<double, LinkedList *, Compare> order_book;
```
The ordered map is what holds the key (the price point where at least
someone has placed an order for an asset). And the value of these
keys is represented by a linked list, holding client orders. 
The compare is one of the things  I stated earlier that this blog won't go over
for the time being, but its simply a way to order the order book.
```cpp
struct client_order {
  int val;
  list items;
  client_order(int order_id) : val(order_id) {}
};
```
A normal implementation of a linked list does heap allocation at least once.
This allocation is simply to add a new node in the list. Obviously,
asking the kernel to find some memory, send the address back for where
your linked list node will fit and then putting the memory in the address
takes time, especially when we are dealing with order books, which are
constantly working with new keys for our map. A normal implementation
of this would look like as above.

```cpp
void append(int order_id) /* you can replace order_id with order info, im using
                            an integer here just to simplify */ {
  if (!head) {
    head = new Node(order_id);
    return;
  }

  auto curr = head;
  while (curr->right) {
    curr = curr->right;
  }
  curr->right = new Node(order_id);
  curr->right->left = curr;
}
```
### The saviour for this comes from ... the kernel 
Intrusive linked lists, a mess to understand as it looks
almost identical to what linked lists look like. The best 
explanation I could give for them is, instead of having
an object which points to the next object. The object itself,
contains the thing that points to the next object's pointer.
As you can see, there is no need for heap allocation here.
Below is an implementation of this and the way you append to
it.

```cpp
// The "pointers", you can also think of these as carabiners.
struct list {
  list *right = nullptr;
  list *left = nullptr; 
};

struct client_order {
  int val;
  list items;
  client_order(int order_id) : val(order_id) {}
};
```

And the magic happens here, a little knowledge of how
memory works underneath is necessary here.

```cpp
// This is what it would look like with client order instead of an integer
void append(client_order *order) {
  list *curr = tail.left; // i decide to keep track of tail and head to avoid iterating the whole list
  curr->right = &order->items;
  order->items.left = curr;
  order->items.right = &tail;
  tail.left = &order->items;
}
//No heap allocation :)
```
And to delete everything once we are done using them (realistically this
never happens as you probably never want to fully delete the whole
order book, but you get an idea how removing singular orders works)
```cpp
~linked_list() {
  list *curr = head.right;
  while (curr != &tail) {
    auto next = curr->right;
    client_order *order = /* we point to the carabiner, not the actual object address start initially */
        (client_order *)((char *)curr - offsetof(client_order, items)); 
    delete order;
    curr = next;
  }
}
```

## Lets give up everything and go against Hedges using ILLs?

Please, no. This is just a way to build your own order
books in case you aren't rich to pay the subscription fees
of the brokers for the order books, as each level of an order
book gives more information regarding that book, it also
goes up in price. How I use this in my trading strategies in
case said strategies use the order book at all, is by
initially giving a cooldown to the trader and in the meantime
building the orderbook via information you can find from the broker
such as how many shares are being sold right now by this price etc.


## Performance between the two based on tests.

All tests here were run without any compiler optimization, 
and I use Catch2 testing suite for calling the test
functions which is how the seed is generated. Additionally, the code
below is what I use to test and how I got the numbers in the first place.

```cpp
TEST_CASE("Order book performance test", "[order-book-perf]") {
  { // perf test
    trader::orderbook::NonIntrusiveOrderBook a;
    trader::orderbook::IntrusiveOrderBook a1;

    int magic_seed = 42;
    int prices_val = 1000;
    int orders_per_price = 500;
    std::default_random_engine e1(magic_seed);
    std::uniform_real_distribution<double> uniform_dist(1, 100);

    std::vector<double> prices;
    prices.reserve(prices_val * orders_per_price);
    for (int i = 0; i < prices_val * orders_per_price; i++) {
      prices.push_back(std::round(uniform_dist(e1) * 100.0) / 100.0);
    }
    std::uniform_int_distribution<int> id_dist(
        0, (prices_val * orders_per_price) - 1);

    auto get_random_id = [&e1, &id_dist]() -> int { return id_dist(e1); };

    int price_idx = 0;

    std::chrono::time_point t = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < prices_val; i++) {
      for (int j = 0; j < orders_per_price; j++) {
        double p = prices[price_idx++];
        a.append(p);
        if (j % 4 == 0)
          a.remove(p, get_random_id());
      }
    }

    std::chrono::time_point t1 = std::chrono::high_resolution_clock::now();
    price_idx = 0;

    for (int i = 0; i < prices_val; i++) {
      for (int j = 0; j < orders_per_price; j++) {
        double p = prices[price_idx++];
        a1.append(p);
        if (j % 4 == 0)
          a1.remove(p, get_random_id());
      }
    }
    std::chrono::time_point t2 = std::chrono::high_resolution_clock::now();
    auto d =
        std::chrono::duration_cast<std::chrono::milliseconds>(t1 - t).count();
    auto d1 =
        std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count();

    std::cout << "Intrusive: " << d1 << "ms | Non-Intrusive: " << d << "ms\n";
  }
}
```


For 1000 different price vals and each
val entry has another 500 orders with that price 

```
Intrusive: 188ms | Non-Intrusive: 353ms
```

For 1000 different price vals and each val
entry has another 1000 orders with that price

```
Intrusive: 485ms | Non-Intrusive: 1333ms
```


Biggest differnce can be seen here, 
5000 different price values by 1000 order per price.

```
Intrusive: 26940ms | Non-Intrusive: 99379ms
```

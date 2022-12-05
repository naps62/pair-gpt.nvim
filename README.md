# pAIr

A GPT-assisted pair programming partner.

# Examples

`cargo run -- --lang rust --prompt "hello world" --color write`

Result:

```rust
fn main() {
    println!("Hello World!");
}
```

---

`cargo run -- --lang javascript --prompt "fetch 10 latest tweets with #portugal" --color write`

Result:

```rust
// Use the TwitterAPI library:

const TwitterAPI = require('twitter');

// Create a client object with your app's crendentials:

const client = new TwitterAPI({
 consumer_key: 'Your consumer key',
 consumer_secret: 'Your consumer secret',
 access_token_key: 'Your access token key',
 access_token_secret: 'Your access token secret'
});

// Call the GET statuses/search/recent endpoint, including the keyword 'portugal' in the request body:
client.get('statuses/search/recent', {q:'#portugal', count: 10}, function(error, tweets, response) {
  if(error) throw error;

  // Print the 10 latest tweets
  tweets.statuses.forEach(function(tweet) {
    console.log(tweet.text);
  });
});
```

---

`cargo run -- --lang rust --prompt "fn main{println!(\"{} {}\", \"Hello\", \"World\");}" --color refactor`

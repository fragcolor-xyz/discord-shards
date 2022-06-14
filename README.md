# discord-shards

## Ideal Shards

```clojure
(defwire message-handler
  (Log "New Message"))

(defwire reaction-add-handler
  (Log "New Reaction"))

(Discord.Bot
 :Token .token
 :Prefix "!"
 :Events {:message message-handler
          :reaction-add reaction-add-handler})
```
flow:

get m3u8 playlist from lookmovie2.to/api/v1/security/episode-access?id_episode=$NUMBER_1&hash=$HASH_1&expires=$EXPIRY

this occurs in the player js with 

```javascript
  m = function (d) {
    var c = new HttpClient,
    g = '/api/v1/security/episode-access?id_episode=' +
    window.currentEpisodeID + '&hash=' + window.show_storage.hash + '&expires=' + window.show_storage.expires;
```

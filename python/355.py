import collections
import heapq

class Twitter:

    def __init__(self):
        # user : list tweets
        self.counter = 0
        self.tweetsByUser = collections.defaultdict(list)
        self.follows = collections.defaultdict(set)
        

    def postTweet(self, userId: int, tweetId: int) -> None:
        self.tweetsByUser[userId].append((-self.counter, tweetId))
        self.counter += 1
        

    def getNewsFeed(self, userId: int) -> List[int]:
        tweets = []
        for follow in self.follows:
            tweets.extend(self.tweetsByUser[follow])
        
        heapq.heapify(tweets)
        feed = []
        while len(tweets) > 0 and len(feed) < 10:
            feed.append(heapq.heappop(tweets))

        return feed

    def follow(self, followerId: int, followeeId: int) -> None:
        self.follows[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        self.follows[followeeId].remove(followeeId)

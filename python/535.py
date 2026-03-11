import base64
from uu import encode

# Approach 1:
# Hashing function 
# long URL -> hash -> short URL
# * has collisions

# Approach 2:
# Counter
# long URL -> {long URL: count++} -> hash count -> short URL

class Codec:
    def __init__(self) -> None:
        self.longUrlToCount = {}
        self.shortToLongUrl = {}

    def encode(self, longUrl: str) -> str:
        """Encodes a URL to a shortened URL.
        """
        count = len(self.longUrlToCount)
        if longUrl in self.longUrlToCount:
            count = self.longUrlToCount[longUrl]

        countBytes = count.to_bytes(byteorder="big", signed=False)
        encoded = base64.b64encode(countBytes)
        shortUrl = encoded.decode("ascii")
        self.shortToLongUrl[shortUrl] = longUrl
        self.longUrlToCount[longUrl] = count
        return shortUrl

    def decode(self, shortUrl: str) -> str:
        """Decodes a shortened URL to its original URL.
        """
        return self.shortToLongUrl[shortUrl]


        
if __name__ == "__main__":
    c = Codec()
    # utf8 -> bytes -> ascii
    print(c.encode("https://test.com"))
    # ascii -> bytes -> utf
    print(c.decode(c.encode("https://test.com")))

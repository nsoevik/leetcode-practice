class Logger:

    def __init__(self):
        self.message_to_ts = {}
        

    def shouldPrintMessage(self, timestamp: int, message: str) -> bool:
        if message not in self.message_to_ts:
            self.message_to_ts[message] = ts
            return True

        existing_ts = self.message_to_ts[message]
        if timestamp - existing_ts < 10:
            return False

        self.message_to_ts[message] = timestamp
        return True

        


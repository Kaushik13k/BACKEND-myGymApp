from enums.query_types import QueryType


class QuerySelector:
    def __init__(self, username):
        self.username = username
        self.queries = {
            QueryType.USER_BODY_MEASUREMENTS: {
                "query": "query ($userAvailable: UserAvailable!) { userBodyMeasurements(userInput: $userAvailable) { user { id username email} bodyMeasurements { weight height weist neck shoulders chest leftBicep rightBicep leftForearm rightForearm abdomen hips leftThigh rightThigh leftCalf rightCalf timestamp} } }",
                "variables": {"userAvailable": {"username": self.username}},
            },
            QueryType.USER: {
                "query": "query ($userAvailable: UserAvailable!) { user(userInput: $userAvailable) }",
                "variables": {"userAvailable": {"username": self.username}},
            },
        }

    def get_query(self, selection):
        return self.queries.get(selection)

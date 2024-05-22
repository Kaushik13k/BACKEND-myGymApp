import requests
import json
from utils.query import QuerySelector
from enums.env import EnvironmentVariables


def request_api(username, auth_token, operation_type, query_type):
    base_url = EnvironmentVariables.AUTH_BASE_URL.value
    selector = QuerySelector(username)
    selected_query = selector.get_query(query_type)

    payload = json.dumps(selected_query)
    headers = {
        "Authorization": f"Bearer {auth_token}",
        "Operation-Type": operation_type,
        "Content-Type": "application/json",
    }

    response = requests.request("POST", base_url, headers=headers, data=payload)

    return response

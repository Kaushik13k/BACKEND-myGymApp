import json
import logging
from http import HTTPStatus
from typing import Union
import redis

from starlette.responses import Response
from starlette.requests import Request, HTTPConnection
from starlette_context.middleware import RawContextMiddleware
from starlette_context.errors import MiddleWareValidationError

from utils import request_api
from enums.query_types import QueryType
from utils.tokens import validate_token
from services.redis_configuration import configure_redis


logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
redis_conn = configure_redis()


class AccessTokenMiddleware(RawContextMiddleware):
    async def set_context(self, request: Union[Request, HTTPConnection]) -> dict:
        url_path = request.url.path.lower()
        operation_name = request.headers.get("Operation-name")
        if url_path in ["/health"] or operation_name in ["Token"]:
            return {}
        auth_token = request.headers.get("Authorization").split("Bearer ")[1]
        if auth_token:
            token_decoded = validate_token(auth_token)
            if token_decoded:
                username_key = f"{token_decoded}:is_valid"
                if redis_conn.exists(username_key):

                    is_valid = (
                        json.loads(redis_conn.get(username_key)).get("data").get("user")
                    )
                    logger.info(f"User available in cache: {is_valid}")
                else:
                    is_valid = request_api.request_api(
                        username=token_decoded,
                        auth_token=auth_token,
                        operation_type="availablity",
                        query_type=QueryType.USER,
                    ).json()
                    redis_conn.set(username_key, json.dumps(is_valid))
                    is_valid = is_valid.get("data").get("user")
                    logger.info(f"User available in DB: {is_valid}")

                if not is_valid:
                    raise MiddleWareValidationError(
                        error_response=Response(
                            status_code=HTTPStatus.BAD_REQUEST,
                            content="App not available in DB.",
                        )
                    )
            else:
                raise MiddleWareValidationError(
                    error_response=Response(
                        status_code=HTTPStatus.BAD_REQUEST,
                        content="Couldn't validate token.",
                    )
                )
        else:
            raise MiddleWareValidationError(
                error_response=Response(
                    status_code=HTTPStatus.BAD_REQUEST,
                    content="Token not provided in Headers.",
                )
            )

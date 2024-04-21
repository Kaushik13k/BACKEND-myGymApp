import redis

from enums.env import EnvironmentVariables


def configure_redis():
    host = EnvironmentVariables.REDIS_URL.value
    port = EnvironmentVariables.REDIS_PORT.value
    db = EnvironmentVariables.REDIS_DB.value
    redis_conn = redis.Redis(host=host, port=port, db=db)
    return redis_conn

from enum import Enum
import os


class EnvironmentVariables(Enum):
    POSTGRES_CONNECTION_URL = os.getenv("POSTGRES_CON_STRING")
    JWT_SECRET = os.getenv("SECRET_KEY")
    AUTH_BASE_URL = os.getenv("SECRET_KEY")
    REDIS_URL = os.getenv("REDIS_URL")
    REDIS_PORT = os.getenv("REDIS_PORT")
    REDIS_DB = os.getenv("REDIS_DB")

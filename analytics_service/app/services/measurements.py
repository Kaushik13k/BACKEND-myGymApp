import json
import logging
import datetime
import traceback

from http import HTTPStatus
from fastapi import Request

from utils.filter_data import get_data
from enums.query_types import QueryType
from utils.request_api import request_api
from model.measurements import MeasurementInput
from enums.date_range import DateType, DayRange
from exceptions.exceptions import MuscleGroupGetException
from services.redis_configuration import configure_redis

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

redis_conn = configure_redis()


def fetch_measurements(measurement: MeasurementInput, request: Request):
    try:
        logger.info(f"fetched measurements data for {measurement.username}")
        current_date = datetime.datetime.now()

        days = DayRange[DateType(measurement.time_range).name].value
        logger.info(f"Fetching data for {measurement.username} for {days} days")

        if days is None:
            logger.error(f"Invalid date range {start_date} to {current_date}")
            raise MuscleGroupGetException(
                msg=f"Invalid date range {start_date} to {current_date}"
            )

        start_date = current_date - datetime.timedelta(days=days)
        start_date = start_date.replace(hour=0, minute=0, second=0, microsecond=0)
        current_date = current_date.replace(hour=23, minute=59, second=59, microsecond=999999)

        key = f"{measurement.username}:{start_date.strftime('%Y-%m-%d')}:{current_date.strftime('%Y-%m-%d')}"

        if redis_conn.exists(key):
            logger.info(f"Fetching data for {key} from cache")
            response_data = json.loads(redis_conn.get(key))
            result = get_data(
                response_data, start_date, current_date, measurement.muscle_type
            )

            return result
        else:
            logger.info(f"Fetching data for {key} from API")

            response = request_api(
                username=measurement.username,
                auth_token=request.headers.get("Authorization").split("Bearer ")[1],
                operation_type="measurements",
                query_type=QueryType.USER_BODY_MEASUREMENTS,
            )

            redis_conn.set(key, json.dumps(response.json()))
            if response.status_code == HTTPStatus.OK:
                result = get_data(
                    response.json(), start_date, current_date, measurement.muscle_type
                )

                return result
            else:
                logger.error(
                    f"Key {measurement.muscle_type} not found for date range {start_date} to {current_date}"
                )
            raise MuscleGroupGetException(
                msg=f"Key {measurement.muscle_type} not found for date range {start_date} to {current_date}"
            )
    except Exception as e:
        logger.error(traceback)
        logger.error(f"Failed to get measurements data: {e}")
        raise MuscleGroupGetException(msg=f"There was error getting the data!")

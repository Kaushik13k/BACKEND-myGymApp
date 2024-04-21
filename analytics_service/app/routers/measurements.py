import logging
from fastapi import APIRouter, Request

from utils.responses import success, error
from model.measurements import MeasurementInput
from services.measurements import fetch_measurements


router = APIRouter()

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@router.get(
    "/measurements",
    tags=["Measurements"],
    responses={404: {"description": "Not found"}},
)
async def create_measurement(measurement: MeasurementInput, request: Request):
    try:
        result = fetch_measurements(measurement, request)
        return success(result, message="Measurements fetched successfully")
    except Exception as e:
        logger.error(f"Failed to get measurements data: {e}")
        return error("failed to get measurements data")

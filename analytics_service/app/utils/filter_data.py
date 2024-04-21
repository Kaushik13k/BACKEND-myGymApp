import logging
import pandas as pd


logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def get_data(response_data, start_date, end_date, muscle_type):
    if "data" in response_data and response_data["data"]:
        df = pd.DataFrame.from_dict(
            response_data["data"]["userBodyMeasurements"]["bodyMeasurements"]
        )

        df["timestamp"] = pd.to_datetime(df["timestamp"], unit="s")

        mask = (df["timestamp"] >= start_date) & (df["timestamp"] <= end_date)
        filtered_data = df.loc[mask, [muscle_type, "timestamp"]]
        filtered_data["timestamp"] = filtered_data["timestamp"].astype("int64") // 10**9

        if filtered_data.empty:
            logger.error(
                f"No data found for muscle type '{muscle_type}' between {start_date} and {end_date}"
            )
        reshaped_data = filtered_data.melt(
            id_vars="timestamp", var_name="muscle_type", value_name="value"
        )
        filtered_data = reshaped_data.to_dict("records")

    return filtered_data

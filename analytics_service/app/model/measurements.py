from pydantic import BaseModel
from sqlalchemy.types import Integer, VARCHAR, FLOAT
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column


from postgresdb.connection import Base


class MeasurementInput(BaseModel):
    username: str
    muscle_type: str
    time_range: str

import os
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base
from enums.env import EnvironmentVariables

Base = declarative_base()

DATABASE_URL = EnvironmentVariables.POSTGRES_CONNECTION_URL.value

engine = create_engine(DATABASE_URL)
Session = sessionmaker(bind=engine)

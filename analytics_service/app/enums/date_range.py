from enum import Enum


class DateType(Enum):
    DAILY = "daily"
    WEEKLY = "weekly"
    MONTHLY = "monthly"
    THREE_MONTHS = "three_months"
    YEARLY = "yearly"


class DayRange(Enum):
    DAILY = 0
    WEEKLY = 7
    MONTHLY = 30
    THREE_MONTHS = 90
    YEARLY = 365

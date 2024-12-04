#!/usr/bin/env -S uv run
# /// script
# dependencies = [
#   "requests>=2.32.0",
#   "typer>=0.13.0",
#   "rich>=13.9.4"
# ]
# ///

import os
import re
import requests
import typer
from pathlib import Path
from time import sleep
from datetime import datetime, timedelta
from rich.console import Console
from rich.logging import RichHandler
import logging
from enum import Enum

# Set up rich console and logging
console = Console()


class LogLevel(str, Enum):
    DEBUG = "DEBUG"
    INFO = "INFO"
    WARNING = "WARNING"
    ERROR = "ERROR"
    CRITICAL = "CRITICAL"


def configure_logging(level: LogLevel):
    """
    Configure the logging level.

    Args:
        level (LogLevel): The logging level to set.
    """
    logging.basicConfig(
        level=level.value,
        format="%(message)s",
        datefmt="[%X]",
        handlers=[RichHandler()],
    )
    global log
    log = logging.getLogger("rich")


def validate_year(year: str) -> str:
    """
    Validate that the year is a 4-digit number.

    Args:
        year (str): The year to validate

    Raises:
        typer.BadParameter: If the year is not a 4-digit number.
    """
    if not year.isdigit() or len(year) != 4:
        raise typer.BadParameter("Year must be a 4-digit number.")
    return year


def validate_day(day: str) -> str:
    """
    Validate that the day is in the format 'day-x' where x is a number.

    Args:
        day (str): The day to validate

    Raises:
        typer.BadParameter: If the day is not in the correct format.
    """
    if not re.match(r"day-\d+", day):
        raise typer.BadParameter(
            "Day must be in the format 'day-x' where x is a number."
        )
    return day


def get_aoc_input(
    year: str = typer.Argument(
        ..., help="Year of the Advent of Code event", callback=validate_year
    ),
    day: str = typer.Argument(
        ..., help="Day of the Advent of Code event", callback=validate_day
    ),
    cwd: Path = typer.Option(
        Path.cwd(), help="Current working directory to save the input files"
    ),
    session: str = typer.Option(
        None,
        help="Session cookie for authentication (optional, can be read from AOC_SESSION environment variable)",
    ),
    wait: bool = typer.Option(
        True, help="Wait if the input is not ready (returns 404)"
    ),
    timeout: int = typer.Option(
        None,
        help="Timeout in seconds to wait for the input to be ready (default is infinite)",
    ),
    log_level: LogLevel = typer.Option(
        LogLevel.INFO, help="Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)"
    ),
):
    """
    Fetch the Advent of Code input for a given year and day, and save it to files.
    """
    configure_logging(log_level)

    if session is None:
        session = os.getenv("AOC_SESSION")
        if session is None:
            raise ValueError(
                "Session cookie must be provided either as an argument or through the AOC_SESSION environment variable"
            )

    day_number = day.split("-")[1]

    url = f"https://adventofcode.com/{year}/day/{day_number}/input"
    log.debug(f"Sending to `{url}`")

    headers = {"Cookie": f"session={session}"}

    end_time = datetime.now() + timedelta(seconds=timeout) if timeout else None

    while True:
        try:
            log.info(f"Fetching input files for {year}/{day}")
            response = requests.get(url, headers=headers)
            response.raise_for_status()
            log.info(f"Retrieved input files for {year}/{day}")
            input_data = response.text
            break
        except requests.exceptions.HTTPError as e:
            if response.status_code == 404:
                if not wait:
                    log.error(
                        "Input not ready (404). Exiting because --wait flag is not set."
                    )
                    exit(1)
                if end_time and datetime.now() >= end_time:
                    log.error("Timeout exceeded. Please try again later.")
                    exit(1)
                if end_time:
                    remaining_time = (end_time - datetime.now()).total_seconds()
                    sleep_time = min(3, remaining_time)
                    if sleep_time > 0:
                        log.warning("Input not ready (404). Retrying in 3 seconds...")
                    sleep(sleep_time)
                else:
                    log.warning("Input not ready (404). Retrying in 3 seconds...")
                    sleep(3)
            else:
                raise e

    for filename in ["input1.txt", "input2.txt"]:
        file_path = cwd / year / day / filename
        file_path.parent.mkdir(parents=True, exist_ok=True)
        with open(file_path, "w") as file:
            file.write(input_data)
            log.info(f"Wrote '{filename}' for {year}/{day}")
            log.debug(f"Wrote {file_path}")


if __name__ == "__main__":
    typer.run(get_aoc_input)

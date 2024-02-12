from typing import List

from enum import Enum


class LocationStatus:
    FOUND = 1
    UNDER_INVESTIGATION = 2


class EntryPoint:
    name: str
    latitude: float
    longitude: float
    description: str


class Note:
    name: str
    timestamp: str
    body: str


class Url:
    name: str
    url: str


class TargetLocation:
    name: str
    google_maps_location: str
    entry_points: List[EntryPoint]
    notes: List[Note]
    urls: List[Url]
    status: LocationStatus

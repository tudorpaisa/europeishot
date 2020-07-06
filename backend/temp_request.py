import json
import os
import requests

from time import sleep

API_KEY = os.getenv("OPENWEATHER_API_KEY")

if API_KEY:
    pass
else:
    raise ValueError("Unable to retrieve API key")


def get_cities() -> dict:
    with open("data.json", "r") as f:
        return json.loads(f.read())


def get_city_temp(city: str) -> dict:
    print(city)
    sleep(1)
    req = requests.get("https://api.openweathermap.org/data/2.5/weather?q=" +
                       city + "&appid=" + API_KEY + "&units=metric")
    try:
        return req.json()
    except ValueError:
        return {}


def check_temps(cities: dict) -> dict:
    temps = [get_city_temp(i["asciiname"]) for i in cities["data"]]
    temps = [i for i in temps
             if i.get("main", {}).get("temp")]  #remove None vals
    return sorted(temps, key=lambda i: i["main"]["temp"], reverse=True)


cities = get_cities()

temps = check_temps(cities)

with open("/tmp/eu-temperatures.json", "w") as f:
    f.write(json.dumps({"temps": temps}, indent=2))

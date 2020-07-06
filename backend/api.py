import json
import requests

from time import sleep
from flask import Flask
from flask_cors import CORS

global app
app = Flask(__name__)
CORS(app, resources={r'/*': {'origins': '*'}})


@app.route('/temps/')
def send_temperatures():
    with open("/tmp/eu-temperatures.json", "r") as f:
        temps = json.loads(f.read())
    return temps


if __name__ == "__main__":
    app.run(host="0.0.0.0")

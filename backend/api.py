import json
import requests

from time import sleep
from flask import Flask, jsonify
from flask_cors import CORS, cross_origin

global app
app = Flask(__name__)
CORS(app, resources={r'/*': {'origins': '*'}})


@app.route('/temps/', methods=["GET"])
def send_temperatures():
    with open("/tmp/eu-temperatures.json", "r") as f:
        temps = json.loads(f.read())
    response = jsonify(temps)
    response.headers.add('Access-Control-Allow-Origin', '*')
    return response


if __name__ == "__main__":
    app.run(host="0.0.0.0")

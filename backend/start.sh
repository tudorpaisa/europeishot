#!/bin/bash
python3 ./temp_request.py
gunicorn -b 0.0.0.0:5000 app:app

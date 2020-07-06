#!/bin/bash
python3 ./temp_request.py
gunicorn -b 127.0.0.1:5000 app:app

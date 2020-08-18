FROM python:3-buster
EXPOSE 3000
WORKDIR /usr/src/app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD [ "python", "./json_sample_server.py" ]

from flask import Flask,request
from datetime import datetime
app = Flask(__name__)
port=3000
@app.route("/hi")
def hello_world():
    result={
        'success':True,
        'message': "hello world!",
        'timestamp':datetime.now().timestamp(),
        'request': f"{request.url}",
    }
    return result
if __name__=='__main__':
    app.run(port=port)
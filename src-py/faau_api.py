import base64
import time
from os import remove

from flask import Flask, request
from utils import faau, scanner

scanner.preload_model()
app = Flask(__name__)


@app.route("/start_analysis", methods=["POST"])
def start_analysis():
    data = request.form
    debug = data.get("debug") == "true"
    b64img = data.get("b64img")
    print(f"[PYTHON]: Analysis debug: {b64img}")
    if not b64img:
        return "Missing Required Params", 400
    imgbuf = base64.b64decode(b64img)
    ftempname = f"tempimg_{int(time.time())}.jpg"
    open(ftempname, "wb").write(imgbuf)
    try:
        scan_res = scanner.analyze_frame(ftempname, debug)
        # NOTE: we only need the first face
        return scan_res[0]
    except Exception as e:
        return f"Failed to analyze frame\nERROR: {e}", 500
    finally:
        remove(ftempname)


if __name__ == "__main__":
    app.run(port=6969)

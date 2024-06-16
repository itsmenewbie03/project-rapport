import importlib

from flask import Flask, request

# NOTE: what the fuck is this abomination?
recorder = importlib.import_module("recorder", "src-py")
faau = importlib.import_module("faau", "src-py")
faau.preload_model()
app = Flask(__name__)


@app.route("/start_recording", methods=["POST"])
def start_recording():
    data = request.form
    fname = data.get("feedback_id")
    if not fname:
        return "Missing Required Params", 400
    fname = f"{fname}.mp4"
    is_recording = recorder.start_capture(fname)
    if not is_recording:
        return "Recorder busy...", 400
    return "Recording started successfully"


@app.route("/start_analysis", methods=["POST"])
def start_analysis():
    data = request.form
    feedback_id = data.get("feedback_id")
    if not feedback_id:
        return "Missing Required Params", 400
    is_started = faau.start_analysis(feedback_id)
    if not is_started:
        return "Faau busy...", 400
    return "Analysis started successfully"


@app.route("/stop_recording")
def stop_recording():
    is_stopped = recorder.stop_capture()
    if not is_stopped:
        return "Not recording", 400
    return "Recording stopped successfully"


@app.route("/clear_recording", methods=["POST"])
def clear_recording():
    data = request.form
    fname = data.get("feedback_id")
    if not fname:
        return "Missing Required Params", 400
    fname = f"{fname}.mp4"
    is_cleared = recorder.clear_capture(fname)
    if not is_cleared:
        return "Failed to clear recording", 400
    return "Recording cleared successfully"


if __name__ == "__main__":
    app.run()

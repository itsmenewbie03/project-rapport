from flask import Flask, request
from utils import faau, recorder

app = Flask(__name__)


app.route("/start_recording", methods=["POST"])


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
    debug = data.get("debug") == "true"
    print(f"[PYTHON]: Analysis debug: {debug}")
    if not feedback_id:
        return "Missing Required Params", 400
    is_started = faau.start_analysis(feedback_id, debug)
    if not is_started:
        return "Faau busy...", 400
    return "Analysis started successfully"


@app.route("/take_photo", methods=["POST"])
def take_photo():
    data = request.form
    feedback_id = data.get("feedback_id")
    quality = data.get("quality")
    print(f"[PYTHON]: Take Photo for {feedback_id} for {quality}")
    if not feedback_id:
        return "Missing Required Params", 400
    photo = faau.take_photo(feedback_id, quality)
    print(f"[PYTHON]: PHOTO TAKEN: {photo}")
    if isinstance(photo, bool) and not photo:
        return "Failed to take photo", 400
    return photo


@app.route("/poll_analysis", methods=["POST"])
def poll_analysis():
    data = request.form
    feedback_id = data.get("feedback_id")
    debug = data.get("debug") == "true"
    print(f"[PYTHON]: Poll Analysis debug: {debug}")
    if not feedback_id:
        return "Missing Required Params", 400
    analysis_result = faau.poll_analysis(feedback_id, debug)
    if not analysis_result:
        return "Not yet done!", 400
    return analysis_result


@app.route("/stop_recording")
def stop_recording():
    is_stopped = faau.stop_analysis()
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
    is_cleared = faau.stop_analysis()
    if not is_cleared:
        return "Failed to clear recording", 400
    return "Recording cleared successfully"


if __name__ == "__main__":
    app.run()
